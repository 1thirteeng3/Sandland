import type { CanvasNodeDTO, CanvasEdgeDTO, ViewportStateDTO } from '../types/canvas';
import type { AssetRefDto } from '../types/vault';
import { loadBoardTopology, saveBoardTopology } from '../services/canvasService';
import { vaultService } from '../services/vaultService';
import { vaultStore } from './vault.svelte';

export class CanvasStore {
  workspaceId = $state<string>('default-workspace');
  viewport = $state<ViewportStateDTO>({ x: 0, y: 0, zoom: 1.0 });
  nodes = $state<CanvasNodeDTO[]>([]);
  edges = $state<CanvasEdgeDTO[]>([]);
  selectedNodeId = $state<string | null>(null);
  editingNodeId = $state<string | null>(null);
  currentRevision = $state<number | null>(null);

  private autoSaveTimer: any = null;

  selectedNode = $derived(this.nodes.find((n) => n.id === this.selectedNodeId) ?? null);
  editingNode = $derived(this.nodes.find((n) => n.id === this.editingNodeId) ?? null);
  nodeCount = $derived(this.nodes.length);

  async loadWorkspace(wsId: string) {
    this.workspaceId = wsId;
    try {
      const topology = await loadBoardTopology(wsId);
      if (topology.viewport) {
        this.viewport = topology.viewport;
      }
      if (Array.isArray(topology.nodes) && topology.nodes.length > 0) {
        this.nodes = topology.nodes;
        this.edges = topology.edges || [];
      } else if (this.nodes.length === 0) {
        // Inicializa com nós padrão apenas na primeira execução se cofre estiver virgem
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
            fromSide: 'right',
            toSide: 'left',
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

  scheduleAutoSave() {
    if (this.autoSaveTimer) {
      clearTimeout(this.autoSaveTimer);
    }
    // Invariante constitucional: janela máxima de perda de digitação <= 500ms
    this.autoSaveTimer = setTimeout(() => {
      this.persistTopology();
    }, 450);
  }

  async persistTopology() {
    vaultStore.setSyncState('saving');
    const topoPayload = {
      workspaceId: this.workspaceId,
      viewport: this.viewport,
      nodes: this.nodes,
      edges: this.edges,
      updatedAt: Math.floor(Date.now() / 1000),
    };

    const relPath = `workspaces/${this.workspaceId}/topology.json`;
    const jsonStr = JSON.stringify(topoPayload, null, 2);

    try {
      // 1. Tenta gravar via protocolo atômico em duas fases com journal e snapshot (Sprint 01)
      const newRev = await vaultService.commitDocument(
        relPath,
        jsonStr,
        this.currentRevision ?? undefined
      );
      this.currentRevision = newRev;
      vaultStore.setSyncState('synced');
      // Atualiza contador de eventos no header
      vaultStore.refreshStatus();
    } catch (err: any) {
      const errStr = String(err?.message || err);
      if (errStr.includes('RevisionConflict')) {
        console.warn('[CanvasStore] Conflito de revisão detectado. Recarregando topologia...');
        vaultStore.setSyncState('conflict');
        await this.loadWorkspace(this.workspaceId);
        return;
      }

      // Fallback para canal legado de salvamento de topologia caso commit_document não esteja disponível
      try {
        await saveBoardTopology(this.workspaceId, topoPayload);
        vaultStore.setSyncState('synced');
      } catch (fallbackErr) {
        console.error('[CanvasStore] Falha ao persistir topologia do workspace:', fallbackErr);
        vaultStore.setSyncState('error');
      }
    }
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
    const newZoom = Math.min(Math.max(oldZoom * deltaZoom, 0.15), 2.5);
    
    // Zoom em torno do cursor do mouse
    const worldX = (screenX - this.viewport.x) / oldZoom;
    const worldY = (screenY - this.viewport.y) / oldZoom;

    this.viewport.x = screenX - worldX * newZoom;
    this.viewport.y = screenY - worldY * newZoom;
    this.viewport.zoom = newZoom;
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
      this.scheduleAutoSave();
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
      width: 260,
      height: 150,
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
    this.edges = this.edges.filter((e) => e.sourceNodeId !== id && e.targetNodeId !== id);
    if (this.selectedNodeId === id) this.selectedNodeId = null;
    if (this.editingNodeId === id) this.editingNodeId = null;
    this.scheduleAutoSave();
  }

  startEditing(nodeId: string) {
    this.editingNodeId = nodeId;
  }

  stopEditing() {
    this.editingNodeId = null;
  }
}

export const canvasStore = new CanvasStore();
