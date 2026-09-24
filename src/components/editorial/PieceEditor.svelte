<script lang="ts">
  import { pieceStore } from '../../stores/piece.svelte';
  import CitationBadge from './CitationBadge.svelte';
  import PieceExportModal from './PieceExportModal.svelte';

  let isDragOver = $state(false);
  let textareaEl: HTMLTextAreaElement | null = $state(null);

  function handleTitleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    pieceStore.updateTitle(target.value);
  }

  function handleBodyInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    pieceStore.updateBody(target.value);
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (!textareaEl) return;
    // Atalho Ctrl+S para salvar imediatamente
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      pieceStore.saveNow();
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'copy';
    }
    isDragOver = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;

    if (!e.dataTransfer) return;

    let payloadStr = e.dataTransfer.getData('application/json');
    if (!payloadStr) {
      payloadStr = e.dataTransfer.getData('text/plain');
    }

    if (payloadStr) {
      try {
        const data = JSON.parse(payloadStr);
        if (data && (data.id || data.cellId)) {
          // Nó arrastado da Mesa Espacial
          await pieceStore.insertCitationFromNode({
            id: data.id || data.cellId,
            title: data.title || 'Cartão da Mesa',
            content: data.content || data.body || '',
            revision: data.revision ?? 1,
            assetHash: data.assetHash || null,
          });
          return;
        }
      } catch {
        // Se for texto plano
        if (payloadStr.trim()) {
          await pieceStore.insertCitationFromNode({
            id: `manual-${Date.now()}`,
            title: 'Texto Externo',
            content: payloadStr.trim(),
            revision: 1,
          });
        }
      }
    }
  }

  function insertMarkdownSnippet(prefix: string, suffix: string = '') {
    if (!textareaEl || !pieceStore.activePiece) return;
    const start = textareaEl.selectionStart;
    const end = textareaEl.selectionEnd;
    const oldText = pieceStore.activePiece.body || '';
    const selected = oldText.substring(start, end);
    const replacement = prefix + (selected || 'texto') + suffix;
    const newText = oldText.substring(0, start) + replacement + oldText.substring(end);
    pieceStore.updateBody(newText);
    setTimeout(() => {
      if (textareaEl) {
        textareaEl.focus();
        textareaEl.setSelectionRange(start + prefix.length, start + replacement.length - suffix.length);
      }
    }, 10);
  }
</script>

<div
  class="piece-editor-container {isDragOver ? 'drag-over' : ''}"
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  role="region"
  aria-label="Editor de Peça Editorial"
>
  {#if isDragOver}
    <div class="drag-overlay">
      <div class="drag-message">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="7 10 12 15 17 10" />
          <line x1="12" y1="15" x2="12" y2="3" />
        </svg>
        <span>Solte aqui para citar com proveniência imutável (Fork-on-Insert)</span>
      </div>
    </div>
  {/if}

  {#if pieceStore.activePiece}
    <!-- Barra Superior da Peça -->
    <header class="editor-header">
      <div class="header-status">
        <input
          type="text"
          class="title-input"
          value={pieceStore.activePiece.title}
          oninput={handleTitleInput}
          placeholder="Título da Peça..."
        />
        <div class="save-status">
          {#if pieceStore.isSaving}
            <span class="status-saving">Salvando...</span>
          {:else if pieceStore.saveError}
            <span class="status-error" title={pieceStore.saveError}>Erro ao salvar ⚠️</span>
          {:else}
            <span class="status-saved">Salvo canonicamente</span>
          {/if}
        </div>
      </div>

      <div class="editor-actions">
        <button class="tool-btn primary" onclick={() => pieceStore.openExportModal()}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="7 10 12 15 17 10" />
            <line x1="12" y1="15" x2="12" y2="3" />
          </svg>
          Exportar
        </button>
      </div>
    </header>

    <!-- Toolbar de Formatação Rápida -->
    <div class="format-toolbar">
      <button class="format-btn" onclick={() => insertMarkdownSnippet('## ', '')} title="Título 2">
        H2
      </button>
      <button class="format-btn" onclick={() => insertMarkdownSnippet('### ', '')} title="Título 3">
        H3
      </button>
      <button class="format-btn" onclick={() => insertMarkdownSnippet('**', '**')} title="Negrito (Ctrl+B)">
        <strong>B</strong>
      </button>
      <button class="format-btn" onclick={() => insertMarkdownSnippet('*', '*')} title="Itálico (Ctrl+I)">
        <em>I</em>
      </button>
      <button class="format-btn" onclick={() => insertMarkdownSnippet('> ', '')} title="Citação">
        ""
      </button>
      <button class="format-btn" onclick={() => insertMarkdownSnippet('`', '`')} title="Código">
        &lt;/&gt;
      </button>
      <div class="toolbar-divider"></div>
      <span class="metric-text">{pieceStore.activeWordCount} palavras</span>
      <span class="metric-text">{pieceStore.activeCitationCount} citações</span>
    </div>

    <!-- Área de Escrita -->
    <div class="editor-content-wrap">
      <textarea
        bind:this={textareaEl}
        class="body-textarea"
        value={pieceStore.activePiece.body}
        oninput={handleBodyInput}
        onkeydown={handleKeyDown}
        placeholder="Escreva sua síntese editorial aqui... Você pode arrastar cartões da Mesa Espacial diretamente para este editor."
        spellcheck="false"
      ></textarea>
    </div>

    <!-- Painel Inferior de Citações e Proveniência Canônica -->
    {#if pieceStore.activePiece.citations.length > 0}
      <footer class="provenance-footer">
        <div class="footer-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
          </svg>
          <span>Proveniência e Citações Imutáveis ({pieceStore.activePiece.citations.length})</span>
        </div>
        <div class="citations-scroll-list">
          {#each pieceStore.activePiece.citations as cit (cit.id)}
            <CitationBadge citation={cit} />
          {/each}
        </div>
      </footer>
    {/if}
  {:else}
    <div class="no-active-piece">
      <p>Nenhuma peça editorial selecionada.</p>
      <button class="tool-btn primary" onclick={() => pieceStore.createPiece()}>
        Criar Nova Peça
      </button>
    </div>
  {/if}
</div>

<!-- Modal de Exportação -->
<PieceExportModal />

<style>
  .piece-editor-container {
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #0f1217;
    height: 100%;
    overflow: hidden;
  }

  .piece-editor-container.drag-over {
    border: 2px dashed #6366f1;
    background: rgba(99, 102, 241, 0.05);
  }

  .drag-overlay {
    position: absolute;
    inset: 0;
    background: rgba(15, 23, 42, 0.85);
    backdrop-filter: blur(4px);
    z-index: 50;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }

  .drag-message {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    color: #a5b4fc;
    font-size: 1rem;
    font-weight: 500;
  }

  .editor-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 28px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    background: #14171d;
  }

  .header-status {
    flex: 1;
    display: flex;
    align-items: baseline;
    gap: 14px;
    min-width: 0;
  }

  .title-input {
    background: transparent;
    border: none;
    font-size: 1.35rem;
    font-weight: 700;
    color: #f8fafc;
    outline: none;
    width: 65%;
    min-width: 200px;
    letter-spacing: -0.02em;
  }

  .title-input:focus {
    border-bottom: 1px solid #6366f1;
  }

  .save-status {
    font-size: 0.75rem;
    user-select: none;
  }

  .status-saving {
    color: #38bdf8;
  }

  .status-saved {
    color: #10b981;
  }

  .status-error {
    color: #ef4444;
  }

  .editor-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .tool-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 500;
    cursor: pointer;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #cbd5e1;
    transition: all 0.15s;
  }

  .tool-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .tool-btn.primary {
    background: #4f46e5;
    border-color: #6366f1;
    color: white;
  }

  .tool-btn.primary:hover:not(:disabled) {
    background: #4338ca;
  }

  .tool-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .format-toolbar {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 28px;
    background: rgba(255, 255, 255, 0.02);
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }

  .format-btn {
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    color: #94a3b8;
    padding: 3px 8px;
    font-size: 0.76rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .format-btn:hover {
    background: rgba(255, 255, 255, 0.06);
    color: #e2e8f0;
  }

  .toolbar-divider {
    width: 1px;
    height: 16px;
    background: rgba(255, 255, 255, 0.1);
    margin: 0 6px;
  }

  .metric-text {
    font-size: 0.72rem;
    color: #64748b;
    margin-left: 6px;
  }

  .editor-content-wrap {
    flex: 1;
    display: flex;
    overflow: hidden;
    padding: 24px 32px;
  }

  .body-textarea {
    flex: 1;
    width: 100%;
    max-width: 820px;
    margin: 0 auto;
    background: transparent;
    border: none;
    outline: none;
    resize: none;
    color: #e2e8f0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
    font-size: 1.05rem;
    line-height: 1.75;
    letter-spacing: -0.01em;
  }

  .body-textarea::placeholder {
    color: #475569;
  }

  .provenance-footer {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px 28px;
    background: #14171d;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }

  .footer-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.75rem;
    font-weight: 600;
    color: #94a3b8;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .citations-scroll-list {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    max-height: 90px;
    overflow-y: auto;
  }

  .no-active-piece {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    color: #64748b;
  }
</style>
