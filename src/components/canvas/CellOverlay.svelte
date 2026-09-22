<script lang="ts">
  import { canvasStore } from '../../stores/canvas.svelte';
  import { onMount, tick } from 'svelte';

  let title = $state('');
  let content = $state('');
  let titleInputRef = $state<HTMLInputElement | null>(null);

  $effect(() => {
    const node = canvasStore.editingNode;
    if (node) {
      title = node.title || '';
      content = node.content || '';
      tick().then(() => {
        titleInputRef?.focus();
      });
    }
  });

  function saveAndClose() {
    const node = canvasStore.editingNode;
    if (node) {
      canvasStore.updateNodeContent(node.id, title, content);
    }
    canvasStore.stopEditing();
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      saveAndClose();
    } else if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      saveAndClose();
    }
  }

  // Coordenadas projetadas na tela via matriz de câmera da viewport
  let screenLeft = $derived(
    canvasStore.editingNode
      ? canvasStore.viewport.x + canvasStore.editingNode.x * canvasStore.viewport.zoom
      : 0
  );
  let screenTop = $derived(
    canvasStore.editingNode
      ? canvasStore.viewport.y + canvasStore.editingNode.y * canvasStore.viewport.zoom
      : 0
  );
  let screenWidth = $derived(
    canvasStore.editingNode ? canvasStore.editingNode.width * canvasStore.viewport.zoom : 0
  );
  let screenHeight = $derived(
    canvasStore.editingNode ? canvasStore.editingNode.height * canvasStore.viewport.zoom : 0
  );
</script>

{#if canvasStore.editingNode}
  <div
    class="cell-overlay-container"
    style="left: {screenLeft}px; top: {screenTop}px; width: {screenWidth}px; height: {screenHeight}px;"
    onkeydown={handleKeyDown}
    role="dialog"
    tabindex="-1"
    aria-label="Editor de célula em foco"
  >
    <div class="overlay-card">
      <input
        type="text"
        class="title-field"
        placeholder="Título da Célula..."
        bind:this={titleInputRef}
        bind:value={title}
      />
      <textarea
        class="content-field"
        placeholder="Escreva notas em Markdown..."
        bind:value={content}
      ></textarea>
      <div class="overlay-footer">
        <span class="hint">Ctrl+Enter para salvar</span>
        <button class="save-btn" onclick={saveAndClose}>Concluir</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .cell-overlay-container {
    position: absolute;
    pointer-events: auto;
    z-index: 50;
    box-sizing: border-box;
    transform-origin: top left;
  }

  .overlay-card {
    width: 100%;
    height: 100%;
    background: rgba(22, 25, 31, 0.98);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 2px solid var(--accent-blue);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg), 0 0 20px rgba(59, 130, 246, 0.25);
    display: flex;
    flex-direction: column;
    padding: 10px;
    gap: 8px;
  }

  .title-field {
    background: transparent;
    border: none;
    border-bottom: 1px solid var(--border-subtle);
    font-size: 0.875rem;
    font-weight: 700;
    color: var(--text-primary);
    padding: 4px 2px;
    width: 100%;
  }

  .title-field:focus {
    border-bottom-color: var(--accent-blue);
  }

  .content-field {
    flex: 1;
    background: transparent;
    border: none;
    resize: none;
    font-size: 0.8125rem;
    line-height: 1.4;
    color: var(--text-primary);
    font-family: var(--font-sans);
  }

  .overlay-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 4px;
    border-top: 1px solid var(--border-subtle);
  }

  .hint {
    font-size: 0.6875rem;
    color: var(--text-muted);
  }

  .save-btn {
    padding: 4px 10px;
    font-size: 0.75rem;
    font-weight: 600;
    background: var(--accent-blue);
    color: #ffffff;
    border-radius: var(--radius-sm);
    transition: opacity 0.15s;
  }

  .save-btn:hover {
    opacity: 0.9;
  }
</style>
