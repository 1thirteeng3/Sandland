import { Application, Container, Graphics, Text, TextStyle } from 'pixi.js';
import type { CanvasNodeDTO, NodeSide } from '../../types/canvas';
import { canvasStore } from '../../stores/canvas.svelte';

interface NodeGraphicsItem {
  container: Container;
  bg: Graphics;
  titleText: Text;
  contentText: Text;
  portsContainer: Container;
}

export class PixiRenderer {
  private app: Application;
  private containerElement: HTMLElement;
  private worldContainer: Container;
  private edgesGraphics: Graphics;
  private previewGraphics: Graphics;
  private nodesContainer: Container;
  private nodeGraphicsMap: Map<string, NodeGraphicsItem> = new Map();

  private isDraggingNode = false;
  private draggedNodeId: string | null = null;
  private dragOffset = { x: 0, y: 0 };
  private isPanning = false;
  private lastPointer = { x: 0, y: 0 };
  private isSpacePressed = false;

  // Drag-and-connect state (T015)
  private isConnecting = false;
  private connectSourceNodeId: string | null = null;
  private connectFromSide: NodeSide | null = null;
  private connectPointerPos = { x: 0, y: 0 };

  constructor(container: HTMLElement) {
    this.containerElement = container;
    this.app = new Application();
    this.worldContainer = new Container();
    this.edgesGraphics = new Graphics();
    this.previewGraphics = new Graphics();
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
    this.worldContainer.addChild(this.previewGraphics);
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

    // Pan do canvas ou desseleção
    canvas.addEventListener('pointerdown', (e) => {
      if (!this.isDraggingNode && !this.isConnecting) {
        if (e.button === 1 || e.button === 0 || this.isSpacePressed) {
          this.isPanning = true;
          this.lastPointer = { x: e.clientX, y: e.clientY };
          if (this.isSpacePressed) {
            canvas.style.cursor = 'grabbing';
          } else {
            // Desseleciona se clicou no fundo
            canvasStore.selectedNodeId = null;
            canvasStore.selectEdge(null);
          }
        }
      }
    });

    window.addEventListener('pointermove', (e) => {
      const rect = this.containerElement.getBoundingClientRect();
      const screenX = e.clientX - rect.left;
      const screenY = e.clientY - rect.top;
      const worldX = (screenX - canvasStore.viewport.x) / canvasStore.viewport.zoom;
      const worldY = (screenY - canvasStore.viewport.y) / canvasStore.viewport.zoom;

      if (this.isConnecting) {
        this.connectPointerPos = { x: worldX, y: worldY };
        this.renderPreviewEdge();
      } else if (this.isDraggingNode && this.draggedNodeId) {
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

    window.addEventListener('pointerup', (e) => {
      if (this.isConnecting) {
        this.finishConnecting(e);
      }

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

  private createNodeGraphics(node: CanvasNodeDTO): NodeGraphicsItem {
    const nodeContainer = new Container();
    nodeContainer.eventMode = 'static';
    nodeContainer.cursor = 'grab';

    const bg = new Graphics();
    nodeContainer.addChild(bg);

    const titleStyle = new TextStyle({
      fontFamily: 'Inter, -apple-system, sans-serif',
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
      fontFamily: 'Inter, -apple-system, sans-serif',
      fontSize: 11,
      fill: '#9da7b8',
      wordWrap: true,
      wordWrapWidth: node.width - 24,
    });
    const contentText = new Text({ text: node.content || '', style: contentStyle });
    contentText.position.set(12, 40);
    nodeContainer.addChild(contentText);

    // Container de portas de ancoragem (T014)
    const portsContainer = new Container();
    nodeContainer.addChild(portsContainer);

    // Interações de clique e arraste no cartão
    let lastClickTime = 0;
    nodeContainer.on('pointerdown', (e) => {
      e.stopPropagation();
      this.isPanning = false;

      // Duplo clique abre o editor overlay imediatamente (T012)
      const now = Date.now();
      if (now - lastClickTime < 300) {
        canvasStore.startEditing(node.id);
        lastClickTime = 0;
        return;
      }
      lastClickTime = now;

      canvasStore.selectedNodeId = node.id;
      canvasStore.selectEdge(null);
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

    return { container: nodeContainer, bg, titleText, contentText, portsContainer };
  }

  private updateNodeGraphics(item: NodeGraphicsItem, node: CanvasNodeDTO) {
    item.container.position.set(node.x, node.y);

    const isSelected = canvasStore.selectedNodeId === node.id;
    const isEditing = canvasStore.editingNodeId === node.id;

    // Se estiver em modo de edição, esconde o texto WebGL para não conflitar com o overlay DOM
    item.titleText.visible = !isEditing;
    item.contentText.visible = !isEditing;

    item.titleText.text = node.title || 'Sem Título';
    item.contentText.text = node.content || '';

    // Renderiza fundo do nó
    item.bg.clear();
    item.bg.roundRect(0, 0, node.width, node.height, 8);
    item.bg.fill({ color: 0x16191f, alpha: 0.95 });
    item.bg.stroke({
      color: isSelected ? 0x3b82f6 : 0x272d3a,
      width: isSelected ? 2 : 1,
    });

    // Renderiza portas magnéticas (T014)
    this.renderAnchorPorts(item.portsContainer, node, isSelected);
  }

  // T014: Renderiza as 4 alças magnéticas (Top, Bottom, Left, Right)
  private renderAnchorPorts(container: Container, node: CanvasNodeDTO, isSelected: boolean) {
    container.removeChildren();

    const sides: NodeSide[] = ['Top', 'Right', 'Bottom', 'Left'];
    for (const side of sides) {
      const pos = this.getAnchorLocalPosition(node, side);
      const portG = new Graphics();
      portG.eventMode = 'static';
      portG.cursor = 'crosshair';

      // Ponto de âncora
      portG.circle(pos.x, pos.y, isSelected ? 5 : 4);
      portG.fill({ color: isSelected ? 0x60a5fa : 0x3b82f6, alpha: isSelected ? 0.9 : 0.6 });
      portG.stroke({ color: 0xffffff, width: 1.5, alpha: 0.8 });

      // Inicia drag-and-connect da alça (T015)
      portG.on('pointerdown', (e) => {
        e.stopPropagation();
        this.isConnecting = true;
        this.connectSourceNodeId = node.id;
        this.connectFromSide = side;

        const rect = this.containerElement.getBoundingClientRect();
        const screenX = e.clientX - rect.left;
        const screenY = e.clientY - rect.top;
        this.connectPointerPos = {
          x: (screenX - canvasStore.viewport.x) / canvasStore.viewport.zoom,
          y: (screenY - canvasStore.viewport.y) / canvasStore.viewport.zoom,
        };
      });

      container.addChild(portG);
    }
  }

  // Finaliza a conexão magnética (T015)
  private finishConnecting(e: PointerEvent) {
    if (!this.connectSourceNodeId || !this.connectFromSide) {
      this.isConnecting = false;
      this.previewGraphics.clear();
      return;
    }

    const rect = this.containerElement.getBoundingClientRect();
    const screenX = e.clientX - rect.left;
    const screenY = e.clientY - rect.top;
    const worldX = (screenX - canvasStore.viewport.x) / canvasStore.viewport.zoom;
    const worldY = (screenY - canvasStore.viewport.y) / canvasStore.viewport.zoom;

    // Procura o nó destino sobre o qual o mouse foi solto
    let targetNode: CanvasNodeDTO | null = null;
    for (const node of canvasStore.nodes) {
      if (node.id === this.connectSourceNodeId) continue; // Rejeição de auto-loop
      if (
        worldX >= node.x &&
        worldX <= node.x + node.width &&
        worldY >= node.y &&
        worldY <= node.y + node.height
      ) {
        targetNode = node;
        break;
      }
    }

    if (targetNode) {
      // Determina a porta destino mais próxima do ponto de soltura
      const toSide = this.findClosestSide(targetNode, worldX, worldY);
      canvasStore.createEdge(
        this.connectSourceNodeId,
        targetNode.id,
        this.connectFromSide,
        toSide
      );
    }

    this.isConnecting = false;
    this.connectSourceNodeId = null;
    this.connectFromSide = null;
    this.previewGraphics.clear();
    this.renderEdges();
  }

  private findClosestSide(node: CanvasNodeDTO, x: number, y: number): NodeSide {
    const sides: NodeSide[] = ['Top', 'Bottom', 'Left', 'Right'];
    let bestSide: NodeSide = 'Left';
    let bestDist = Infinity;

    for (const side of sides) {
      const pos = this.getAnchorPosition(node, side);
      const dx = pos.x - x;
      const dy = pos.y - y;
      const dist = dx * dx + dy * dy;
      if (dist < bestDist) {
        bestDist = dist;
        bestSide = side;
      }
    }

    return bestSide;
  }

  // T015: Renderiza a linha elástica de preview durante a criação de conexão
  private renderPreviewEdge() {
    this.previewGraphics.clear();
    if (!this.isConnecting || !this.connectSourceNodeId || !this.connectFromSide) return;

    const sourceNode = canvasStore.nodes.find((n) => n.id === this.connectSourceNodeId);
    if (!sourceNode) return;

    const start = this.getAnchorPosition(sourceNode, this.connectFromSide);
    const end = this.connectPointerPos;

    const dx = end.x - start.x;
    const dy = end.y - start.y;
    const dist = Math.sqrt(dx * dx + dy * dy);
    const handleOffset = Math.min(Math.max(dist * 0.4, 30), 140);

    const cp1 = this.getControlPoint(start, this.connectFromSide, handleOffset);

    this.previewGraphics.stroke({ color: 0x60a5fa, width: 2, alpha: 0.9 });
    this.previewGraphics.moveTo(start.x, start.y);
    this.previewGraphics.bezierCurveTo(cp1.x, cp1.y, end.x, end.y, end.x, end.y);

    this.previewGraphics.circle(start.x, start.y, 4);
    this.previewGraphics.fill({ color: 0x3b82f6 });

    this.previewGraphics.circle(end.x, end.y, 5);
    this.previewGraphics.fill({ color: 0x60a5fa });
  }

  // T016: Renderização de Arestas Bézier cúbicas e setas direcionais
  private renderEdges() {
    this.edgesGraphics.clear();
    const edges = canvasStore.edges;
    const nodeMap = new Map(canvasStore.nodes.map((n) => [n.id, n]));

    for (const edge of edges) {
      const source = nodeMap.get(edge.sourceNodeId);
      const target = nodeMap.get(edge.targetNodeId);
      if (!source || !target) continue;

      const isSelected = canvasStore.selectedEdgeId === edge.id;
      const start = this.getAnchorPosition(source, edge.fromSide);
      const end = this.getAnchorPosition(target, edge.toSide);

      const dx = end.x - start.x;
      const dy = end.y - start.y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      const handleOffset = Math.min(Math.max(dist * 0.4, 40), 160);

      const cp1 = this.getControlPoint(start, edge.fromSide, handleOffset);
      const cp2 = this.getControlPoint(end, edge.toSide, handleOffset);

      // Traçado da curva
      const edgeColor = isSelected ? 0x60a5fa : 0x5271ff;
      const edgeWidth = isSelected ? 3 : 2;

      this.edgesGraphics.stroke({ color: edgeColor, width: edgeWidth, alpha: isSelected ? 1 : 0.85 });
      this.edgesGraphics.moveTo(start.x, start.y);
      this.edgesGraphics.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, end.x, end.y);

      // Círculo na âncora de origem
      this.edgesGraphics.circle(start.x, start.y, 4);
      this.edgesGraphics.fill({ color: 0x3b82f6 });

      // Seta direcional na âncora de destino
      if (edge.directed) {
        this.renderDirectionalArrow(end, edge.toSide, edgeColor);
      }
    }
  }

  private renderDirectionalArrow(pos: { x: number; y: number }, side: NodeSide, color: number) {
    const s = side.toLowerCase();
    const arrowSize = 8;
    let angle = 0;

    if (s === 'left') angle = 0; // Apontando para a direita (entrando pelo lado esquerdo)
    else if (s === 'right') angle = Math.PI; // Apontando para a esquerda
    else if (s === 'top') angle = Math.PI / 2; // Apontando para baixo
    else if (s === 'bottom') angle = -Math.PI / 2; // Apontando para cima

    const tipX = pos.x;
    const tipY = pos.y;
    const leftX = tipX - arrowSize * Math.cos(angle - Math.PI / 6);
    const leftY = tipY - arrowSize * Math.sin(angle - Math.PI / 6);
    const rightX = tipX - arrowSize * Math.cos(angle + Math.PI / 6);
    const rightY = tipY - arrowSize * Math.sin(angle + Math.PI / 6);

    this.edgesGraphics.moveTo(tipX, tipY);
    this.edgesGraphics.lineTo(leftX, leftY);
    this.edgesGraphics.lineTo(rightX, rightY);
    this.edgesGraphics.closePath();
    this.edgesGraphics.fill({ color });
  }

  private getAnchorLocalPosition(node: CanvasNodeDTO, side: NodeSide) {
    const s = side.toLowerCase();
    switch (s) {
      case 'left':
        return { x: 0, y: node.height / 2 };
      case 'right':
        return { x: node.width, y: node.height / 2 };
      case 'top':
        return { x: node.width / 2, y: 0 };
      case 'bottom':
        return { x: node.width / 2, y: node.height };
      default:
        return { x: 0, y: 0 };
    }
  }

  private getAnchorPosition(node: CanvasNodeDTO, side: NodeSide) {
    const local = this.getAnchorLocalPosition(node, side);
    return { x: node.x + local.x, y: node.y + local.y };
  }

  private getControlPoint(pos: { x: number; y: number }, side: NodeSide, offset: number) {
    const s = side.toLowerCase();
    switch (s) {
      case 'left':
        return { x: pos.x - offset, y: pos.y };
      case 'right':
        return { x: pos.x + offset, y: pos.y };
      case 'top':
        return { x: pos.x, y: pos.y - offset };
      case 'bottom':
        return { x: pos.x, y: pos.y + offset };
      default:
        return { x: pos.x, y: pos.y };
    }
  }

  // T024: Frustum Culling AABB & T025: Level of Detail (LOD) a 60 FPS
  public applyLODAndCulling() {
    const zoom = canvasStore.viewport.zoom;
    const { width, height } = this.app.screen;

    // Viewport AABB em coordenadas do mundo
    const worldLeft = -canvasStore.viewport.x / zoom;
    const worldTop = -canvasStore.viewport.y / zoom;
    const worldRight = (width - canvasStore.viewport.x) / zoom;
    const worldBottom = (height - canvasStore.viewport.y) / zoom;

    for (const [id, item] of this.nodeGraphicsMap.entries()) {
      const node = canvasStore.nodes.find((n) => n.id === id);
      if (!node) continue;

      // 1. Frustum Culling AABB (T024)
      const isVisible =
        node.x + node.width >= worldLeft &&
        node.x <= worldRight &&
        node.y + node.height >= worldTop &&
        node.y <= worldBottom;

      item.container.visible = isVisible;
      if (!isVisible) continue;

      // 2. 3-Stage LOD (T025)
      if (zoom >= 0.8) {
        // LOD Alto: Renderiza título e texto completo
        item.titleText.visible = canvasStore.editingNodeId !== id;
        item.contentText.visible = canvasStore.editingNodeId !== id;
        item.portsContainer.visible = true;
      } else if (zoom >= 0.35) {
        // LOD Médio: Renderiza apenas título
        item.titleText.visible = true;
        item.contentText.visible = false;
        item.portsContainer.visible = false;
      } else {
        // LOD Macro: Proxy card sólido simplificado sem texto para maximizar performance
        item.titleText.visible = false;
        item.contentText.visible = false;
        item.portsContainer.visible = false;
      }
    }
  }

  public destroy() {
    this.app.destroy(true, { children: true, texture: true });
  }
}
