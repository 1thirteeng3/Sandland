<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { canvasStore } from '../../stores/canvas.svelte';
  import { PixiRenderer } from './PixiRenderer';
  import CellOverlay from './CellOverlay.svelte';
  import CanvasControls from './CanvasControls.svelte';

  let containerRef = $state<HTMLDivElement | null>(null);
  let renderer = $state<PixiRenderer | null>(null);

  onMount(async () => {
    if (!containerRef) return;

    renderer = new PixiRenderer(containerRef);
    await renderer.init();
    await canvasStore.loadWorkspace('default-workspace');
    renderer.updateData();

    window.addEventListener('keydown', handleGlobalKeyDown);
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleGlobalKeyDown);
    renderer?.destroy();
  });

  function handleGlobalKeyDown(e: KeyboardEvent) {
    // Se estiver editando em um input ou textarea, não processa atalhos de deleção
    if (
      canvasStore.editingNodeId ||
      document.activeElement?.tagName === 'INPUT' ||
      document.activeElement?.tagName === 'TEXTAREA'
    ) {
      return;
    }

    // T018: Deleção por teclado para aresta ou nó selecionado
    if (e.key === 'Delete' || e.key === 'Backspace') {
      if (canvasStore.selectedEdgeId) {
        e.preventDefault();
        canvasStore.deleteEdge(canvasStore.selectedEdgeId);
      } else if (canvasStore.selectedNodeId) {
        e.preventDefault();
        canvasStore.deleteNode(canvasStore.selectedNodeId);
      }
    } else if (e.key === 'Enter') {
      // Abre o editor overlay no nó selecionado
      if (canvasStore.selectedNodeId) {
        e.preventDefault();
        canvasStore.startEditing(canvasStore.selectedNodeId);
      }
    }
  }

  // Reage a mudanças nos nós, arestas ou na viewport para atualizar o PixiRenderer
  $effect(() => {
    // Registra dependências reativas
    void canvasStore.nodes;
    void canvasStore.edges;
    void canvasStore.viewport;
    void canvasStore.editingNodeId;
    void canvasStore.selectedNodeId;
    void canvasStore.selectedEdgeId;

    if (renderer) {
      renderer.updateData();
    }
  });

  function handleCanvasDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'copy';
    }
  }

  function handleCanvasDrop(e: DragEvent) {
    e.preventDefault();
    if (!e.dataTransfer || !containerRef) return;

    try {
      const raw = e.dataTransfer.getData('application/json');
      if (raw) {
        const item = JSON.parse(raw);
        const rect = containerRef.getBoundingClientRect();
        const screenX = e.clientX - rect.left;
        const screenY = e.clientY - rect.top;
        const worldX = (screenX - canvasStore.viewport.x) / canvasStore.viewport.zoom;
        const worldY = (screenY - canvasStore.viewport.y) / canvasStore.viewport.zoom;
        canvasStore.addNode(item.title || 'Item Ingerido', item.content || item.summary || '', worldX, worldY);
      }
    } catch (err) {
      console.error('Falha ao soltar item no canvas:', err);
    }
  }
</script>

<div
  class="canvas-viewport"
  bind:this={containerRef}
  ondragover={handleCanvasDragOver}
  ondrop={handleCanvasDrop}
  role="region"
  aria-label="Mesa de Trabalho Canvas"
>
  <!-- T022: Banner de Alerta de Conflito de Concorrência Otimista (OCC) -->
  {#if canvasStore.revisionConflict}
    <aside class="conflict-banner" role="alert" aria-live="assertive">
      <div class="conflict-info">
        <svg class="warning-icon" viewBox="0 0 24 24" width="18" height="18" stroke="currentColor" fill="none" stroke-width="2">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" y1="8" x2="12" y2="12"></line>
          <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        <span><strong>Conflito de Revisão (OCC):</strong> O arquivo em disco foi atualizado externamente.</span>
      </div>
      <button class="reload-btn" onclick={() => canvasStore.reloadAfterConflict()}>
        Recarregar e Sincronizar
      </button>
    </aside>
  {/if}

  <CellOverlay />
  <CanvasControls />
</div>

<style>
  .canvas-viewport {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: radial-gradient(circle at 50% 50%, #16191f 0%, #0d0f12 100%);
  }

  .conflict-banner {
    position: absolute;
    top: 16px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 100;
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 10px 18px;
    background: rgba(239, 68, 68, 0.92);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid rgba(254, 202, 202, 0.4);
    border-radius: var(--radius-md, 8px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    color: #ffffff;
    font-size: 0.8125rem;
    animation: slideDown 0.25s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translate(-50%, -10px);
    }
    to {
      opacity: 1;
      transform: translate(-50%, 0);
    }
  }

  .conflict-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .warning-icon {
    flex-shrink: 0;
  }

  .reload-btn {
    padding: 4px 10px;
    font-size: 0.75rem;
    font-weight: 600;
    background: #ffffff;
    color: #b91c1c;
    border: none;
    border-radius: var(--radius-sm, 4px);
    cursor: pointer;
    white-space: nowrap;
    transition: background 0.15s, transform 0.1s;
  }

  .reload-btn:hover {
    background: #fee2e2;
  }

  .reload-btn:active {
    transform: scale(0.97);
  }
</style>
