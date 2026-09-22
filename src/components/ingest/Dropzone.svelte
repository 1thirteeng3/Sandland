<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { ingestStore } from '../../stores/ingest.svelte';
  import { isTauri } from '../../services/ipc';

  let isDragging = $state(false);
  let urlInput = $state('');
  let isSubmitting = $state(false);
  let feedbackMessage = $state<string | null>(null);
  let fileInputRef: HTMLInputElement | undefined = $state();

  onMount(() => {
    let unlisten: (() => void) | undefined;
    if (isTauri()) {
      try {
        getCurrentWebview()
        .onDragDropEvent(async (event) => {
          if (event.payload.type === 'over') {
            isDragging = true;
          } else if (event.payload.type === 'leave') {
            isDragging = false;
          } else if (event.payload.type === 'drop') {
            isDragging = false;
            const paths = event.payload.paths;
            for (const p of paths) {
              if (p.endsWith('.md') || p.endsWith('.markdown') || p.endsWith('.txt')) {
                try {
                  feedbackMessage = `Ingerindo "${p}"...`;
                  await ingestStore.addFile(p);
                  feedbackMessage = `Arquivo adicionado com sucesso!`;
                  setTimeout(() => (feedbackMessage = null), 3000);
                } catch (err: any) {
                  const msg = err?.message || (typeof err === 'object' ? JSON.stringify(err) : String(err));
                  feedbackMessage = `Erro: ${msg}`;
                  setTimeout(() => (feedbackMessage = null), 8000);
                }
              }
            }
          }
        })
        .then((fn) => {
          unlisten = fn;
        });
    } catch {
      // Ambiente de desenvolvimento web sem Tauri
    }
  }

    return () => {
      unlisten?.();
    };
  });

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    if (!e.dataTransfer) return;

    const files = Array.from(e.dataTransfer.files);
    for (const file of files) {
      if (file.name.endsWith('.md') || file.name.endsWith('.markdown') || file.name.endsWith('.txt') || file.type.includes('text')) {
        try {
          feedbackMessage = `Ingerindo "${file.name}"...`;
          const content = await file.text();
          await ingestStore.addFileContent(file.name, content);
          feedbackMessage = `"${file.name}" adicionado com sucesso!`;
          setTimeout(() => (feedbackMessage = null), 3000);
        } catch (err: any) {
          const msg = err?.message || (typeof err === 'object' ? JSON.stringify(err) : String(err));
          feedbackMessage = `Erro: ${msg}`;
          setTimeout(() => (feedbackMessage = null), 8000);
        }
      }
    }
  }

  async function handleFileInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;

    const files = Array.from(target.files);
    for (const file of files) {
      try {
        feedbackMessage = `Ingerindo "${file.name}"...`;
        const content = await file.text();
        await ingestStore.addFileContent(file.name, content);
        feedbackMessage = `"${file.name}" adicionado com sucesso!`;
        setTimeout(() => (feedbackMessage = null), 3000);
      } catch (err: any) {
        const msg = err?.message || (typeof err === 'object' ? JSON.stringify(err) : String(err));
        feedbackMessage = `Erro: ${msg}`;
        setTimeout(() => (feedbackMessage = null), 8000);
      }
    }
    target.value = '';
  }

  function triggerFilePicker() {
    fileInputRef?.click();
  }

  async function handleUrlSubmit(e: SubmitEvent) {
    e.preventDefault();
    let url = urlInput.trim();
    if (!url) return;

    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      url = 'https://' + url;
    }

    isSubmitting = true;
    try {
      feedbackMessage = 'Capturando URL...';
      await ingestStore.addUrl(url);
      urlInput = '';
      feedbackMessage = 'Snapshot da URL capturado!';
      setTimeout(() => (feedbackMessage = null), 3000);
    } catch (err: any) {
      const msg = err?.message || (typeof err === 'object' ? JSON.stringify(err) : String(err));
      feedbackMessage = `Erro: ${msg}`;
      setTimeout(() => (feedbackMessage = null), 8000);
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="dropzone-container">
  <input
    type="file"
    accept=".md,.markdown,.txt,text/plain,text/markdown"
    multiple
    style="display: none;"
    bind:this={fileInputRef}
    onchange={handleFileInputChange}
  />

  <div
    class="drop-area"
    class:dragging={isDragging}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
    onclick={triggerFilePicker}
    role="button"
    tabindex="0"
    onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && triggerFilePicker()}
    aria-label="Área de captura e upload de arquivos"
  >
    <div class="icon-circle">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
        <polyline points="17 8 12 3 7 8" />
        <line x1="12" y1="3" x2="12" y2="15" />
      </svg>
    </div>
    <p class="primary-text">Arraste notas Markdown aqui</p>
    <p class="secondary-text">ou clique para selecionar arquivos do computador</p>
  </div>

  <form class="url-input-group" onsubmit={handleUrlSubmit}>
    <input
      type="text"
      placeholder="Capturar URL (ex: https://site.com)..."
      bind:value={urlInput}
      disabled={isSubmitting}
    />
    <button type="submit" disabled={isSubmitting || !urlInput.trim()}>
      {#if isSubmitting}
        ...
      {:else}
        Capturar
      {/if}
    </button>
  </form>

  {#if feedbackMessage}
    <div class="feedback-banner">
      {feedbackMessage}
    </div>
  {/if}
</div>

<style>
  .dropzone-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .drop-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 24px 16px;
    border: 2px dashed var(--border-subtle);
    border-radius: var(--radius-md);
    background: rgba(22, 25, 31, 0.6);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    cursor: pointer;
  }

  .drop-area:hover,
  .drop-area.dragging {
    border-color: var(--accent-blue);
    background: rgba(59, 130, 246, 0.08);
    transform: translateY(-1px);
  }

  .icon-circle {
    width: 44px;
    height: 44px;
    border-radius: var(--radius-full);
    background: var(--bg-surface);
    color: var(--accent-blue);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 10px;
  }

  .primary-text {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .secondary-text {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 4px;
  }

  .url-input-group {
    display: flex;
    gap: 8px;
  }

  .url-input-group input {
    flex: 1;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    font-size: 0.8125rem;
    color: var(--text-primary);
    transition: border-color 0.2s;
  }

  .url-input-group input:focus {
    border-color: var(--accent-blue);
  }

  .url-input-group button {
    padding: 8px 14px;
    background: var(--accent-blue);
    color: #ffffff;
    font-size: 0.8125rem;
    font-weight: 500;
    border-radius: var(--radius-sm);
    transition: opacity 0.2s;
  }

  .url-input-group button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .feedback-banner {
    font-size: 0.75rem;
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    background: var(--bg-surface);
    color: var(--accent-cyan);
    border: 1px solid var(--border-subtle);
    animation: fadeIn 0.2s ease-in;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
