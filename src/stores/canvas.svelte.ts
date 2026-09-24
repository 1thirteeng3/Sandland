import type { CanvasNodeDTO, CanvasEdgeDTO, ViewportStateDTO, NodeSide, BoardTopologyDTO } from '../types/canvas';
import type { AssetRefDto } from '../types/vault';
import {
  loadBoardTopology,
  saveBoardTopology,
  saveWorkspaceCell,
} from '../services/canvasService';
import { vaultService } from '../services/vaultService';
import { vaultStore } from './vault.svelte';

export class CanvasStore {
  workspaceId = $state<string>('default-workspace');
  viewport = $state<ViewportStateDTO>({ x: 0, y: 0, zoom: 1.0 });
  nodes = $state<CanvasNodeDTO[]>([]);
  edges = $state<CanvasEdgeDTO[]>([]);
  selectedNodeId = $state<string | null>(null);
  selectedEdgeId = $state<string | null>(null);
  editingNodeId = $state<string | null>(null);
  editingTitle = $state<string>('');
  editingContent = $state<string>('');
  currentRevision = $state<number | null>(null);
  revisionConflict = $state<boolean>(false);

  private autoSaveTimer: any = null;

  selectedNode = $derived(this.nodes.find((n) => n.id === this.selectedNodeId) ?? null);
  selectedEdge = $derived(this.edges.find((e) => e.id === this.selectedEdgeId) ?? null);
  editingNode = $derived(this.nodes.find((n) => n.id === this.editingNodeId) ?? null);
  nodeCount = $derived(this.nodes.length);

  async loadWorkspace(wsId: string) {
    this.workspaceId = wsId;
    this.revisionConflict = false;
    try {
      const topology = await loadBoardTopology(wsId);
      if (topology.viewport) {
        this.viewport = topology.viewport;
      }
      if (topology.revision !== undefined) {
        this.currentRevision = topology.revision;
      }
      if (Array.isArray(topology.nodes) && topology.nodes.length > 0) {
        this.nodes = topology.nodes;
        this.edges = topology.edges || [];
      } else if (this.nodes.length === 0) {
        // Inicializa com nós padrão na primeira execução
        this.nodes = [
          {
            id: 'node-welcome',
            title: 'Bem-vindo ao Sandland',
            content: 'Dê um duplo clique nesta célula para editar via overlay DOM projetado em Svelte 5.',
            x: 200,
            y: 200,
            width: 280,
            height: 160,
            nodeType: 'text',
          },
          {
            id: 'node-arch',
            title: 'Arquitetura Local-First',
            content: 'Persistência contínua com SQLite index.db e File-as-Truth em disco Markdown.',
            x: 560,
            y: 200,
            width: 280,
            height: 160,
            nodeType: 'text',
          },
        ];
        this.edges = [
          {
            id: 'edge-welcome-arch',
            sourceNodeId: 'node-welcome',
            targetNodeId: 'node-arch',
            fromSide: 'Right',
            toSide: 'Left',
            directed: true,
          },
        ];
        this.scheduleAutoSave();
      }
    } catch (err) {
      console.warn('Iniciando workspace local ou offline:', err);
    }
  }

  loadTopology(wsId: string = 'default-workspace') {
    return this.loadWorkspace(wsId);
  }

  scheduleAutoSave(delayMs: number = 400) {
    if (this.autoSaveTimer) {
      clearTimeout(this.autoSaveTimer);
    }
    // Invariante constitucional: persistência com debounce de 400ms
    this.autoSaveTimer = setTimeout(() => {
      this.persistTopology();
    }, delayMs);
  }

  async persistTopology() {
    vaultStore.setSyncState('saving');
    const topoPayload: BoardTopologyDTO = {
      workspaceId: this.workspaceId,
      viewport: this.viewport,
      nodes: this.nodes,
      edges: this.edges,
      revision: this.currentRevision ?? 1,
      updatedAt: Math.floor(Date.now() / 1000),
    };

    try {
      // Gravação atômica via IPC com OCC revision
      const newRev = await saveBoardTopology(
        this.workspaceId,
        topoPayload,
        this.currentRevision
      );
      this.currentRevision = newRev;
      this.revisionConflict = false;
      vaultStore.setSyncState('synced');
      vaultStore.refreshStatus();
    } catch (err: any) {
      const errStr = String(err?.message || err);
      if (errStr.includes('RevisionConflict') || errStr.includes('Conflito de revisão')) {
        console.warn('[CanvasStore] Conflito de revisão detectado (OCC). Notificando usuário...');
        this.revisionConflict = true;
        vaultStore.setSyncState('conflict');
        return;
      }

      // Fallback para gravação via commit_document se aplicável
      try {
        const relPath = `workspaces/${this.workspaceId}/board.canvas.json`;
        const jsonStr = JSON.stringify(topoPayload, null, 2);
        const newRev = await vaultService.commitDocument(
          relPath,
          jsonStr,
          this.currentRevision ?? undefined
        );
        this.currentRevision = newRev;
        this.revisionConflict = false;
        vaultStore.setSyncState('synced');
        vaultStore.refreshStatus();
      } catch (fallbackErr) {
        console.error('[CanvasStore] Falha ao persistir topologia do workspace:', fallbackErr);
        vaultStore.setSyncState('error');
      }
    }
  }

  async reloadAfterConflict() {
    this.revisionConflict = false;
    await this.loadWorkspace(this.workspaceId);
  }

  setViewport(x: number, y: number, zoom: number) {
    this.viewport = { x, y, zoom: Math.min(Math.max(zoom, 0.1), 3.0) };
    this.scheduleAutoSave();
  }

  pan(dx: number, dy: number) {
    this.viewport.x += dx;
    this.viewport.y += dy;
  }

  zoomAt(deltaZoom: number, screenX: number, screenY: number) {
    const oldZoom = this.viewport.zoom;
    const newZoom = Math.min(Math.max(oldZoom * deltaZoom, 0.1), 3.0);

    // Zoom em torno do cursor do mouse
    const worldX = (screenX - this.viewport.x) / oldZoom;
    const worldY = (screenY - this.viewport.y) / oldZoom;

    this.viewport.x = screenX - worldX * newZoom;
    this.viewport.y = screenY - worldY * newZoom;
    this.viewport.zoom = newZoom;
    this.scheduleAutoSave();
  }

  zoomIn() {
    const centerX = window.innerWidth / 2;
    const centerY = window.innerHeight / 2;
    this.zoomAt(1.2, centerX, centerY);
  }

  zoomOut() {
    const centerX = window.innerWidth / 2;
    const centerY = window.innerHeight / 2;
    this.zoomAt(0.8, centerX, centerY);
  }

  resetZoom() {
    this.viewport.zoom = 1.0;
    this.scheduleAutoSave();
  }

  fitToView() {
    if (this.nodes.length === 0) {
      this.viewport = { x: 0, y: 0, zoom: 1.0 };
      this.scheduleAutoSave();
      return;
    }

    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
    for (const node of this.nodes) {
      minX = Math.min(minX, node.x);
      minY = Math.min(minY, node.y);
      maxX = Math.max(maxX, node.x + node.width);
      maxY = Math.max(maxY, node.y + node.height);
    }

    const padding = 100;
    const boxW = Math.max(maxX - minX + padding * 2, 400);
    const boxH = Math.max(maxY - minY + padding * 2, 300);

    const screenW = window.innerWidth;
    const screenH = window.innerHeight;

    const zoom = Math.min(Math.max(Math.min(screenW / boxW, screenH / boxH), 0.2), 1.5);
    const centerX = (minX + maxX) / 2;
    const centerY = (minY + maxY) / 2;

    this.viewport = {
      x: screenW / 2 - centerX * zoom,
      y: screenH / 2 - centerY * zoom,
      zoom,
    };
    this.scheduleAutoSave();
  }

  updateNodePosition(id: string, x: number, y: number) {
    const node = this.nodes.find((n) => n.id === id);
    if (node) {
      node.x = x;
      node.y = y;
      this.scheduleAutoSave();
    }
  }

  updateNodeContent(id: string, title: string, content: string) {
    const node = this.nodes.find((n) => n.id === id);
    if (node) {
      node.title = title;
      node.content = content;
      this.scheduleAutoSave(400);

      // Persistência canônica da célula em workspaces/<id>/cells/<node_id>.md
      const cellId = node.id.replace(/^node-/, 'cell-');
      saveWorkspaceCell(this.workspaceId, cellId, content, {
        title,
        nodeId: node.id,
        itemId: node.itemId,
      }).catch((err) => {
        console.warn('[CanvasStore] Falha ao persistir nota canônica da célula:', err);
      });
    }
  }

  addNode(title: string, content: string, x: number, y: number): CanvasNodeDTO {
    const newNode: CanvasNodeDTO = {
      id: `node-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
      title,
      content,
      nodeType: 'text',
      x,
      y,
      width: 280,
      height: 160,
    };
    this.nodes = [...this.nodes, newNode];
    this.scheduleAutoSave();
    return newNode;
  }

  addAssetNode(title: string, assetRef: AssetRefDto, x?: number, y?: number): CanvasNodeDTO {
    const nodeX = x ?? (Math.abs(this.viewport.x) + 220 + (this.nodes.length % 5) * 40);
    const nodeY = y ?? (Math.abs(this.viewport.y) + 200 + (this.nodes.length % 5) * 30);
    const newNode: CanvasNodeDTO = {
      id: `node-asset-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
      title,
      content: assetRef.canonicalUri,
      nodeType: 'asset',
      assetHash: assetRef.sha256Hash,
      assetExtension: assetRef.extension,
      x: nodeX,
      y: nodeY,
      width: 280,
      height: 220,
    };
    this.nodes = [...this.nodes, newNode];
    this.scheduleAutoSave();
    return newNode;
  }

  createAssetNode(title: string, assetRef: AssetRefDto, x?: number, y?: number): CanvasNodeDTO {
    return this.addAssetNode(title, assetRef, x, y);
  }

  promoteItemToNode(item: any, x?: number, y?: number): CanvasNodeDTO {
    const nodeX = x ?? (Math.abs(this.viewport.x) + 200 + (this.nodes.length % 5) * 40);
    const nodeY = y ?? (Math.abs(this.viewport.y) + 180 + (this.nodes.length % 5) * 30);

    const newNode: CanvasNodeDTO = {
      id: `node-ingest-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
      itemId: item.id,
      localCellPath: item.canonicalUri || item.vaultPath,
      title: item.title,
      content: item.summary || item.contentSnippet || item.title,
      nodeType: item.itemType === 'web_snapshot' ? 'note' : 'text',
      x: nodeX,
      y: nodeY,
      width: 280,
      height: 160,
      colorPreset: 'blue',
    };

    this.nodes = [...this.nodes, newNode];
    this.scheduleAutoSave();
    return newNode;
  }

  deleteNode(id: string) {
    this.nodes = this.nodes.filter((n) => n.id !== id);
    // Exclusão em cascata das arestas vinculadas
    this.edges = this.edges.filter((e) => e.sourceNodeId !== id && e.targetNodeId !== id);
    if (this.selectedNodeId === id) this.selectedNodeId = null;
    if (this.editingNodeId === id) this.editingNodeId = null;
    this.scheduleAutoSave();
  }

  // Métodos de Gestão de Arestas (T017)
  createEdge(
    sourceNodeId: string,
    targetNodeId: string,
    fromSide: NodeSide,
    toSide: NodeSide,
    label?: string
  ): CanvasEdgeDTO | null {
    // 1. Rejeição de auto-loop (Spec & Data-Model invariant)
    if (sourceNodeId === targetNodeId) {
      console.warn('[CanvasStore] Auto-loop rejeitado entre nó e ele mesmo.');
      return null;
    }

    // 2. Prevenção de aresta duplicada
    const exists = this.edges.some(
      (e) =>
        e.sourceNodeId === sourceNodeId &&
        e.targetNodeId === targetNodeId &&
        e.fromSide.toLowerCase() === fromSide.toLowerCase() &&
        e.toSide.toLowerCase() === toSide.toLowerCase()
    );
    if (exists) {
      return null;
    }

    const newEdge: CanvasEdgeDTO = {
      id: `edge-${Date.now()}-${Math.random().toString(36).substring(2, 6)}`,
      sourceNodeId,
      targetNodeId,
      fromSide,
      toSide,
      label,
      directed: true,
    };

    this.edges = [...this.edges, newEdge];
    this.scheduleAutoSave();
    return newEdge;
  }

  deleteEdge(id: string) {
    this.edges = this.edges.filter((e) => e.id !== id);
    if (this.selectedEdgeId === id) this.selectedEdgeId = null;
    this.scheduleAutoSave();
  }

  updateEdgeLabel(id: string, label: string) {
    const edge = this.edges.find((e) => e.id === id);
    if (edge) {
      edge.label = label;
      this.scheduleAutoSave();
    }
  }

  selectEdge(id: string | null) {
    this.selectedEdgeId = id;
    if (id) {
      this.selectedNodeId = null;
    }
  }

  // Gestão de Edição sob Demanda (T009)
  startEditing(nodeId: string) {
    const node = this.nodes.find((n) => n.id === nodeId);
    if (node) {
      this.editingNodeId = nodeId;
      this.editingTitle = node.title || '';
      this.editingContent = node.content || '';
      this.selectedNodeId = nodeId;
    }
  }

  stopEditing() {
    if (this.editingNodeId) {
      const node = this.nodes.find((n) => n.id === this.editingNodeId);
      if (node) {
        node.title = this.editingTitle;
        node.content = this.editingContent;
        this.updateNodeContent(node.id, this.editingTitle, this.editingContent);
      }
    }
    this.editingNodeId = null;
  }
}

export const canvasStore = new CanvasStore();
