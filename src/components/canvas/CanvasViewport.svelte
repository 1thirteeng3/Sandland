<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { canvasStore } from '../../stores/canvas.svelte';
  import { PixiRenderer } from './PixiRenderer';
  import CellOverlay from './CellOverlay.svelte';
  import CanvasControls from './CanvasControls.svelte';

  let containerRef: HTMLDivElement | null = null;
  let renderer: PixiRenderer | null = null;

  onMount(async () => {
    if (!containerRef) return;

    renderer = new PixiRenderer(containerRef);
    await renderer.init();
    await canvasStore.loadWorkspace('default-workspace');
    renderer.updateData();
  });

  onDestroy(() => {
    renderer?.destroy();
  });

  // Reage a mudanças nos nós ou na viewport para atualizar o PixiRenderer
  $effect(() => {
    // Registra dependências reativas
    const _nodes = canvasStore.nodes;
    const _viewport = canvasStore.viewport;
    const _editing = canvasStore.editingNodeId;
    const _selected = canvasStore.selectedNodeId;

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
        canvasStore.addNode(item.title, item.content, worldX, worldY);
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
</style>
