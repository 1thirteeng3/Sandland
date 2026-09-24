<script lang="ts">
  import { canvasStore } from '../../stores/canvas.svelte';
  import { pieceStore } from '../../stores/piece.svelte';
  import { vaultService } from '../../services/vaultService';
  import { tick } from 'svelte';

  let titleInputRef = $state<HTMLInputElement | null>(null);
  let assetDataUrl = $state<string | null>(null);

  $effect(() => {
    const node = canvasStore.editingNode;
    if (node) {
      assetDataUrl = null;

      // Se for um nó de mídia CAS, lê os bytes base64 para preview
      if (node.nodeType === 'asset' && node.assetHash) {
        vaultService
          .readAsset(node.assetHash)
          .then((b64) => {
            const mime =
              node.assetExtension === 'png'
                ? 'image/png'
                : node.assetExtension === 'svg'
                  ? 'image/svg+xml'
                  : 'image/jpeg';
            assetDataUrl = `data:${mime};base64,${b64}`;
          })
          .catch((err) => {
            console.warn('[CellOverlay] Falha ao carregar preview do CAS:', err);
          });
      }

      tick().then(() => {
        titleInputRef?.focus();
      });
    }
  });

  function saveAndClose() {
    canvasStore.stopEditing();
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.stopPropagation();
      saveAndClose();
    } else if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      e.stopPropagation();
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
    canvasStore.editingNode ? Math.max(canvasStore.editingNode.width * canvasStore.viewport.zoom, 280) : 0
  );
  let screenHeight = $derived(
    canvasStore.editingNode ? Math.max(canvasStore.editingNode.height * canvasStore.viewport.zoom, 180) : 0
  );
</script>

{#if canvasStore.editingNode}
  <!-- Backdrop invisível para capturar clique fora e salvar (blur) -->
  <div
    class="overlay-backdrop"
    onclick={saveAndClose}
    onkeydown={(e) => e.key === 'Escape' && saveAndClose()}
    tabindex="-1"
    role="button"
    aria-label="Fechar editor"
  ></div>

  <div
    class="cell-overlay-container"
    style="left: {screenLeft}px; top: {screenTop}px; width: {screenWidth}px; height: {screenHeight}px;"
    onkeydown={handleKeyDown}
    role="dialog"
    tabindex="-1"
    aria-label="Editor de célula modular em foco"
    onclick={(e) => e.stopPropagation()}
  >
    <div class="overlay-card">
      <div class="card-header">
        <input
          type="text"
          class="title-field"
          placeholder="Título da Célula..."
          bind:this={titleInputRef}
          bind:value={canvasStore.editingTitle}
        />
        <span class="cell-badge">{canvasStore.editingNode.nodeType || 'note'}</span>
      </div>

      {#if canvasStore.editingNode.nodeType === 'asset'}
        <div class="asset-preview-container">
          {#if assetDataUrl}
            <img src={assetDataUrl} alt={canvasStore.editingTitle} class="asset-image" />
          {:else}
            <div class="asset-loading">Carregando do CAS ({canvasStore.editingNode.assetHash?.slice(0, 10)}...)...</div>
          {/if}
        </div>
      {/if}

      <textarea
        class="content-field"
        placeholder="Escreva notas em Markdown canônico..."
        bind:value={canvasStore.editingContent}
      ></textarea>

      <div class="overlay-footer">
        <span class="hint">Esc ou Ctrl+Enter para salvar</span>
        <div class="btn-group">
          <button
            class="cite-btn"
            draggable="true"
            ondragstart={(e) => {
              if (canvasStore.editingNode && e.dataTransfer) {
                e.dataTransfer.setData(
                  'application/json',
                  JSON.stringify({
                    id: canvasStore.editingNode.id,
                    title: canvasStore.editingTitle,
                    content: canvasStore.editingContent,
                    revision: canvasStore.editingNode.revision || 1,
                    assetHash: canvasStore.editingNode.assetHash,
                  })
                );
                e.dataTransfer.effectAllowed = 'copy';
              }
            }}
            onclick={async () => {
              if (canvasStore.editingNode) {
                await pieceStore.insertCitationFromNode({
                  id: canvasStore.editingNode.id,
                  title: canvasStore.editingTitle,
                  content: canvasStore.editingContent,
                  revision: canvasStore.editingNode.revision || 1,
                  assetHash: canvasStore.editingNode.assetHash,
                });
                if (pieceStore.viewMode === 'board') {
                  pieceStore.setViewMode('split');
                }
              }
            }}
            title="Arrastar ou clicar para citar na Peça Editorial (Fork-on-Insert)"
          >
            <span class="btn-icon">📋</span>
            <span>Citar na Peça</span>
          </button>
          <button class="delete-btn" onclick={() => {
            if (canvasStore.editingNode) {
              const id = canvasStore.editingNode.id;
              canvasStore.stopEditing();
              canvasStore.deleteNode(id);
            }
          }}>Excluir</button>
          <button class="save-btn" onclick={saveAndClose}>Concluir</button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay-backdrop {
    position: fixed;
    inset: 0;
    z-index: 45;
    background: transparent;
  }

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
    background: rgba(18, 21, 28, 0.98);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 2px solid var(--accent-blue, #3b82f6);
    border-radius: var(--radius-md, 8px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.6), 0 0 24px rgba(59, 130, 246, 0.3);
    display: flex;
    flex-direction: column;
    padding: 12px;
    gap: 8px;
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    border-bottom: 1px solid var(--border-subtle, #272d3a);
    padding-bottom: 6px;
  }

  .title-field {
    background: transparent;
    border: none;
    font-size: 0.9375rem;
    font-weight: 700;
    color: var(--text-primary, #f0f3f8);
    padding: 2px 0;
    width: 100%;
    font-family: inherit;
  }

  .title-field:focus {
    outline: none;
  }

  .cell-badge {
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--accent-blue, #3b82f6);
    background: rgba(59, 130, 246, 0.12);
    padding: 2px 6px;
    border-radius: 4px;
    white-space: nowrap;
  }

  .asset-preview-container {
    max-height: 120px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-tertiary, #12151c);
    border-radius: var(--radius-sm, 6px);
    overflow: hidden;
  }

  .asset-image {
    max-height: 120px;
    width: auto;
    object-fit: contain;
  }

  .asset-loading {
    font-size: 11px;
    color: var(--text-muted, #717d91);
    padding: 12px;
  }

  .content-field {
    flex: 1;
    min-height: 70px;
    background: transparent;
    border: none;
    resize: none;
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--text-primary, #f0f3f8);
    font-family: var(--font-sans, 'Inter', -apple-system, BlinkMacSystemFont, sans-serif);
    outline: none;
  }

  .overlay-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 6px;
    border-top: 1px solid var(--border-subtle, #272d3a);
  }

  .hint {
    font-size: 0.6875rem;
    color: var(--text-muted, #717d91);
  }

  .btn-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .delete-btn {
    padding: 4px 8px;
    font-size: 0.75rem;
    font-weight: 500;
    background: transparent;
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: var(--radius-sm, 4px);
    cursor: pointer;
    transition: background 0.15s;
  }

  .delete-btn:hover {
    background: rgba(239, 68, 68, 0.1);
  }

  .cite-btn {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 4px 10px;
    font-size: 0.75rem;
    font-weight: 500;
    background: rgba(99, 102, 241, 0.15);
    color: #a5b4fc;
    border: 1px solid rgba(99, 102, 241, 0.4);
    border-radius: var(--radius-sm, 4px);
    cursor: grab;
    transition: all 0.15s;
    user-select: none;
  }

  .cite-btn:hover {
    background: rgba(99, 102, 241, 0.25);
    border-color: rgba(99, 102, 241, 0.7);
    color: #ffffff;
  }

  .cite-btn:active {
    cursor: grabbing;
  }

  .save-btn {
    padding: 4px 12px;
    font-size: 0.75rem;
    font-weight: 600;
    background: var(--accent-blue, #3b82f6);
    color: #ffffff;
    border: none;
    border-radius: var(--radius-sm, 4px);
    cursor: pointer;
    transition: opacity 0.15s;
  }

  .save-btn:hover {
    opacity: 0.9;
  }
</style>
