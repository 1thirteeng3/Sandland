<script lang="ts">
  import { pieceStore } from '../../stores/piece.svelte';
  import { compilePieceExport } from '../../services/pieceService';

  let includeReferences = $state(true);
  let compiledText = $state('');
  let isCopied = $state(false);
  let isCompiling = $state(false);

  $effect(() => {
    if (pieceStore.isExportModalOpen && pieceStore.activePiece) {
      loadExportText();
    }
  });

  async function loadExportText() {
    if (!pieceStore.activePiece) return;
    isCompiling = true;
    try {
      compiledText = await compilePieceExport(
        pieceStore.workspaceId,
        pieceStore.activePiece.id,
        includeReferences
      );
    } catch (e) {
      console.error('Falha ao compilar exportação:', e);
      compiledText = pieceStore.activePiece.body;
    } finally {
      isCompiling = false;
    }
  }

  async function handleToggleReferences(val: boolean) {
    includeReferences = val;
    await loadExportText();
  }

  async function handleCopy() {
    if (!compiledText) return;
    try {
      await navigator.clipboard.writeText(compiledText);
      isCopied = true;
      setTimeout(() => {
        isCopied = false;
      }, 2000);
    } catch (err) {
      console.error('Erro ao copiar:', err);
    }
  }

  function handleDownload() {
    if (!compiledText || !pieceStore.activePiece) return;
    const blob = new Blob([compiledText], { type: 'text/markdown;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${pieceStore.activePiece.slug || 'export'}.md`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  function handleClose() {
    pieceStore.closeExportModal();
  }
</script>

{#if pieceStore.isExportModalOpen}
  <div
    class="modal-backdrop"
    onclick={handleClose}
    onkeydown={(e) => e.key === 'Escape' && handleClose()}
    role="presentation"
  >
    <div
      class="modal-container"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      tabindex="-1"
      aria-label="Exportar Peça Editorial"
    >
      <div class="modal-header">
        <div class="title-wrap">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="7 10 12 15 17 10" />
            <line x1="12" y1="15" x2="12" y2="3" />
          </svg>
          <h3>Exportar Peça Editorial</h3>
        </div>
        <button class="close-btn" onclick={handleClose}>✕</button>
      </div>

      <div class="modal-body">
        <div class="export-options">
          <label class="option-pill {includeReferences ? 'active' : ''}">
            <input
              type="radio"
              name="export-ref"
              checked={includeReferences}
              onchange={() => handleToggleReferences(true)}
            />
            <span>Com Apêndice de Referências</span>
          </label>
          <label class="option-pill {!includeReferences ? 'active' : ''}">
            <input
              type="radio"
              name="export-ref"
              checked={!includeReferences}
              onchange={() => handleToggleReferences(false)}
            />
            <span>Markdown Limpo (Apenas Corpo)</span>
          </label>
        </div>

        <div class="preview-container">
          <div class="preview-header">
            <span>Pré-visualização do Arquivo Exportado</span>
            {#if isCompiling}
              <span class="compiling-indicator">Compilando...</span>
            {/if}
          </div>
          <pre class="preview-content">{compiledText}</pre>
        </div>
      </div>

      <div class="modal-footer">
        <button class="footer-btn secondary" onclick={handleClose}>
          Fechar
        </button>
        <div class="action-buttons">
          <button class="footer-btn secondary copy-btn {isCopied ? 'copied' : ''}" onclick={handleCopy}>
            {#if isCopied}
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              Copiado!
            {:else}
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
              </svg>
              Copiar Markdown
            {/if}
          </button>
          <button class="footer-btn primary" onclick={handleDownload}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            Baixar Arquivo (.md)
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    z-index: 1100;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
  }

  .modal-container {
    width: 680px;
    max-width: 95vw;
    background: #181b21;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.6);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: popIn 0.18s ease-out;
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .title-wrap {
    display: flex;
    align-items: center;
    gap: 10px;
    color: #e2e8f0;
  }

  .title-wrap h3 {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 600;
  }

  .close-btn {
    background: none;
    border: none;
    color: #94a3b8;
    font-size: 1.1rem;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.1);
  }

  .modal-body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .export-options {
    display: flex;
    gap: 12px;
  }

  .option-pill {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    background: rgba(30, 34, 42, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    font-size: 0.82rem;
    color: #94a3b8;
    cursor: pointer;
    transition: all 0.15s;
  }

  .option-pill input {
    cursor: pointer;
  }

  .option-pill.active {
    background: rgba(99, 102, 241, 0.15);
    border-color: rgba(99, 102, 241, 0.5);
    color: #e0e7ff;
    font-weight: 500;
  }

  .preview-container {
    display: flex;
    flex-direction: column;
    background: #0f1217;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    overflow: hidden;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    font-size: 0.75rem;
    color: #64748b;
  }

  .compiling-indicator {
    color: #38bdf8;
  }

  .preview-content {
    margin: 0;
    padding: 14px;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.8rem;
    line-height: 1.5;
    color: #cbd5e1;
    max-height: 260px;
    overflow-y: auto;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .modal-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(20, 24, 30, 0.9);
  }

  .action-buttons {
    display: flex;
    gap: 10px;
  }

  .footer-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    border-radius: 6px;
    font-size: 0.82rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
    border: none;
  }

  .footer-btn.secondary {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #cbd5e1;
  }

  .footer-btn.secondary:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .copy-btn.copied {
    background: rgba(16, 185, 129, 0.2);
    border-color: rgba(16, 185, 129, 0.5);
    color: #34d399;
  }

  .footer-btn.primary {
    background: #4f46e5;
    color: #fff;
  }

  .footer-btn.primary:hover {
    background: #4338ca;
  }
</style>
