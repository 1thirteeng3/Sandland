import { Application, Container, Graphics, Text, TextStyle } from 'pixi.js';
import type { CanvasNodeDTO, NodeSide } from '../../types/canvas';
import { canvasStore } from '../../stores/canvas.svelte';

export class PixiRenderer {
  private app: Application;
  private containerElement: HTMLElement;
  private worldContainer: Container;
  private edgesGraphics: Graphics;
  private nodesContainer: Container;
  private nodeGraphicsMap: Map<string, { container: Container; bg: Graphics; titleText: Text; contentText: Text }> = new Map();

  private isDraggingNode = false;
  private draggedNodeId: string | null = null;
  private dragOffset = { x: 0, y: 0 };
  private isPanning = false;
  private lastPointer = { x: 0, y: 0 };
  private isSpacePressed = false;

  constructor(container: HTMLElement) {
    this.containerElement = container;
    this.app = new Application();
    this.worldContainer = new Container();
    this.edgesGraphics = new Graphics();
    this.nodesContainer = new Container();
  }

  async init() {
    await this.app.init({
      resizeTo: this.containerElement,
      backgroundColor: 0x0d0f12,
      antialias: true,
      resolution: window.devicePixelRatio || 1,
      autoDensity: true,
    });

    this.app.canvas.style.position = 'absolute';
    this.app.canvas.style.top = '0';
    this.app.canvas.style.left = '0';
    this.app.canvas.style.width = '100%';
    this.app.canvas.style.height = '100%';
    this.app.canvas.style.zIndex = '1';

    this.containerElement.appendChild(this.app.canvas);

    this.worldContainer.addChild(this.edgesGraphics);
    this.worldContainer.addChild(this.nodesContainer);
    this.app.stage.addChild(this.worldContainer);

    this.setupInteraction();
  }

  private setupInteraction() {
    const canvas = this.app.canvas;

    window.addEventListener('keydown', (e) => {
      if (e.code === 'Space') {
        this.isSpacePressed = true;
        canvas.style.cursor = 'grab';
      }
    });

    window.addEventListener('keyup', (e) => {
      if (e.code === 'Space') {
        this.isSpacePressed = false;
        canvas.style.cursor = 'default';
      }
    });

    // Pan do canvas com botão do meio ou arrasto de fundo com botão esquerdo
    canvas.addEventListener('pointerdown', (e) => {
      if (!this.isDraggingNode) {
        if (e.button === 1 || e.button === 0 || this.isSpacePressed) {
          this.isPanning = true;
          this.lastPointer = { x: e.clientX, y: e.clientY };
          if (this.isSpacePressed) {
            canvas.style.cursor = 'grabbing';
          }
        }
      }
    });

    window.addEventListener('pointermove', (e) => {
      if (this.isDraggingNode && this.draggedNodeId) {
        const rect = this.containerElement.getBoundingClientRect();
        const screenX = e.clientX - rect.left;
        const screenY = e.clientY - rect.top;
        const worldX = (screenX - canvasStore.viewport.x) / canvasStore.viewport.zoom;
        const worldY = (screenY - canvasStore.viewport.y) / canvasStore.viewport.zoom;

        const newX = worldX - this.dragOffset.x;
        const newY = worldY - this.dragOffset.y;

        canvasStore.updateNodePosition(this.draggedNodeId, newX, newY);
        this.renderNodes();
        this.renderEdges();
      } else if (this.isPanning) {
        const dx = e.clientX - this.lastPointer.x;
        const dy = e.clientY - this.lastPointer.y;
        this.lastPointer = { x: e.clientX, y: e.clientY };
        canvasStore.pan(dx, dy);
        this.syncCamera();
      }
    });

    window.addEventListener('pointerup', () => {
      if (this.isDraggingNode) {
        this.isDraggingNode = false;
        this.draggedNodeId = null;
      }
      this.isPanning = false;
      if (this.isSpacePressed) {
        canvas.style.cursor = 'grab';
      } else {
        canvas.style.cursor = 'default';
      }
    });

    // Zoom via Scroll Wheel
    canvas.addEventListener('wheel', (e) => {
      e.preventDefault();
      const rect = this.containerElement.getBoundingClientRect();
      const screenX = e.clientX - rect.left;
      const screenY = e.clientY - rect.top;

      const delta = e.deltaY < 0 ? 1.1 : 0.9;
      canvasStore.zoomAt(delta, screenX, screenY);
      this.syncCamera();
      this.applyLODAndCulling();
    });
  }

  public syncCamera() {
    const { x, y, zoom } = canvasStore.viewport;
    this.worldContainer.position.set(x, y);
    this.worldContainer.scale.set(zoom);
  }

  public updateData() {
    this.syncCamera();
    this.renderNodes();
    this.renderEdges();
    this.applyLODAndCulling();
  }

  private renderNodes() {
    const nodes = canvasStore.nodes;
    const currentIds = new Set(nodes.map((n) => n.id));

    // Remove deleted nodes
    for (const [id, item] of this.nodeGraphicsMap.entries()) {
      if (!currentIds.has(id)) {
        this.nodesContainer.removeChild(item.container);
        item.container.destroy({ children: true });
        this.nodeGraphicsMap.delete(id);
      }
    }

    // Upsert nodes
    for (const node of nodes) {
      let item = this.nodeGraphicsMap.get(node.id);
      if (!item) {
        item = this.createNodeGraphics(node);
        this.nodeGraphicsMap.set(node.id, item);
        this.nodesContainer.addChild(item.container);
      }

      this.updateNodeGraphics(item, node);
    }
  }

  private createNodeGraphics(node: CanvasNodeDTO) {
    const nodeContainer = new Container();
    nodeContainer.eventMode = 'static';
    nodeContainer.cursor = 'grab';

    const bg = new Graphics();
    nodeContainer.addChild(bg);

    const titleStyle = new TextStyle({
      fontFamily: 'Inter, sans-serif',
      fontSize: 14,
      fontWeight: 'bold',
      fill: '#f0f3f8',
      wordWrap: true,
      wordWrapWidth: node.width - 24,
    });
    const titleText = new Text({ text: node.title || 'Sem Título', style: titleStyle });
    titleText.position.set(12, 12);
    nodeContainer.addChild(titleText);

    const contentStyle = new TextStyle({
      fontFamily: 'Inter, sans-serif',
      fontSize: 11,
      fill: '#9da7b8',
      wordWrap: true,
      wordWrapWidth: node.width - 24,
    });
    const contentText = new Text({ text: node.content || '', style: contentStyle });
    contentText.position.set(12, 40);
    nodeContainer.addChild(contentText);

    // Node Interaction
    nodeContainer.on('pointerdown', (e) => {
      e.stopPropagation();
      this.isPanning = false;
      canvasStore.selectedNodeId = node.id;
      this.isDraggingNode = true;
      this.draggedNodeId = node.id;

      const rect = this.containerElement.getBoundingClientRect();
      const screenX = e.clientX - rect.left;
      const screenY = e.clientY - rect.top;
      const worldX = (screenX - canvasStore.viewport.x) / canvasStore.viewport.zoom;
      const worldY = (screenY - canvasStore.viewport.y) / canvasStore.viewport.zoom;

      this.dragOffset = {
        x: worldX - node.x,
        y: worldY - node.y,
      };
    });

    // Duplo clique projeta o overlay DOM de edição de texto em Svelte 5 (T039)
    let lastClickTime = 0;
    nodeContainer.on('pointerup', () => {
      const now = Date.now();
      if (now - lastClickTime < 300) {
        canvasStore.startEditing(node.id);
      }
      lastClickTime = now;
    });

    return { container: nodeContainer, bg, titleText, contentText };
  }

  private updateNodeGraphics(
    item: { container: Container; bg: Graphics; titleText: Text; contentText: Text },
    node: CanvasNodeDTO
  ) {
    item.container.position.set(node.x, node.y);

    const isSelected = canvasStore.selectedNodeId === node.id;
    const isEditing = canvasStore.editingNodeId === node.id;

    // Se estiver em modo de edição, esconde o texto WebGL para não conflitar com o overlay DOM
    item.titleText.visible = !isEditing;
    item.contentText.visible = !isEditing;

    item.titleText.text = node.title || 'Sem Título';
    item.contentText.text = node.content || '';

    item.bg.clear();
    item.bg.roundRect(0, 0, node.width, node.height, 8);
    item.bg.fill({ color: 0x16191f, alpha: 0.95 });
    item.bg.stroke({
      color: isSelected ? 0x3b82f6 : 0x2d3442,
      width: isSelected ? 2 : 1,
    });
  }

  private renderEdges() {
    this.edgesGraphics.clear();
    const edges = canvasStore.edges;
    const nodeMap = new Map(canvasStore.nodes.map((n) => [n.id, n]));

    for (const edge of edges) {
      const source = nodeMap.get(edge.sourceNodeId);
      const target = nodeMap.get(edge.targetNodeId);
      if (!source || !target) continue;

      const start = this.getAnchorPosition(source, edge.fromSide);
      const end = this.getAnchorPosition(target, edge.toSide);

      // Curva de Bézier cúbica suave entre âncoras
      const dx = end.x - start.x;
      const dy = end.y - start.y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      const handleOffset = Math.min(Math.max(dist * 0.4, 40), 160);

      const cp1 = this.getControlPoint(start, edge.fromSide, handleOffset);
      const cp2 = this.getControlPoint(end, edge.toSide, handleOffset);

      this.edgesGraphics.stroke({ color: 0x5271ff, width: 2, alpha: 0.8 });
      this.edgesGraphics.moveTo(start.x, start.y);
      this.edgesGraphics.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, end.x, end.y);

      // Círculo na âncora de origem
      this.edgesGraphics.circle(start.x, start.y, 4);
      this.edgesGraphics.fill({ color: 0x3b82f6 });

      // Seta direcional na âncora de destino
      if (edge.directed) {
        this.edgesGraphics.circle(end.x, end.y, 4);
        this.edgesGraphics.fill({ color: 0x5271ff });
      }
    }
  }

  private getAnchorPosition(node: CanvasNodeDTO, side: NodeSide) {
    switch (side) {
      case 'left':
        return { x: node.x, y: node.y + node.height / 2 };
      case 'right':
        return { x: node.x + node.width, y: node.y + node.height / 2 };
      case 'top':
        return { x: node.x + node.width / 2, y: node.y };
      case 'bottom':
        return { x: node.x + node.width / 2, y: node.y + node.height };
    }
  }

  private getControlPoint(pos: { x: number; y: number }, side: NodeSide, offset: number) {
    switch (side) {
      case 'left':
        return { x: pos.x - offset, y: pos.y };
      case 'right':
        return { x: pos.x + offset, y: pos.y };
      case 'top':
        return { x: pos.x, y: pos.y - offset };
      case 'bottom':
        return { x: pos.x, y: pos.y + offset };
    }
  }

  // T042: Frustum Culling & T043: 3-stage Level of Detail (LOD)
  public applyLODAndCulling() {
    const zoom = canvasStore.viewport.zoom;
    const { width, height } = this.app.screen;

    // Converte os limites da tela para coordenadas do mundo
    const worldLeft = -canvasStore.viewport.x / zoom;
    const worldTop = -canvasStore.viewport.y / zoom;
    const worldRight = (width - canvasStore.viewport.x) / zoom;
    const worldBottom = (height - canvasStore.viewport.y) / zoom;

    for (const [id, item] of this.nodeGraphicsMap.entries()) {
      const node = canvasStore.nodes.find((n) => n.id === id);
      if (!node) continue;

      // 1. Frustum Culling
      const isVisible =
        node.x + node.width >= worldLeft &&
        node.x <= worldRight &&
        node.y + node.height >= worldTop &&
        node.y <= worldBottom;

      item.container.visible = isVisible;
      if (!isVisible) continue;

      // 2. 3-Stage LOD
      if (zoom > 0.6) {
        // LOD Alto: Renderiza título e texto completo
        item.titleText.visible = canvasStore.editingNodeId !== id;
        item.contentText.visible = canvasStore.editingNodeId !== id;
      } else if (zoom >= 0.3) {
        // LOD Médio: Renderiza apenas título
        item.titleText.visible = true;
        item.contentText.visible = false;
      } else {
        // LOD Baixo: Proxy card sólido simplificado (sem textos para maximizar draw calls)
        item.titleText.visible = false;
        item.contentText.visible = false;
      }
    }
  }

  public destroy() {
    this.app.destroy(true, { children: true, texture: true });
  }
}
