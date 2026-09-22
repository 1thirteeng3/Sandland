<script lang="ts">
  import { vaultStore } from "../../stores/vault.svelte";
  import { canvasStore } from "../../stores/canvas.svelte";
  import { isTauri } from "../../services/ipc";

  let activeTab = $state<"open" | "create">("open");
  let openPath = $state("");
  let createName = $state("Meu Cofre");
  let createPath = $state("");
  let isSubmitting = $state(false);
  let errorMessage = $state<string | null>(null);

  async function handleOpenVault(path: string) {
    if (!path.trim()) {
      errorMessage = "Informe o caminho da pasta do cofre.";
      return;
    }
    isSubmitting = true;
    errorMessage = null;
    try {
      await vaultStore.openVault(path.trim());
      await canvasStore.loadWorkspace("default-workspace");
    } catch (err: any) {
      errorMessage = err?.message || String(err);
    } finally {
      isSubmitting = false;
    }
  }

  async function handleCreateVault() {
    if (!createPath.trim()) {
      errorMessage = "Informe a pasta onde o cofre será criado.";
      return;
    }
    if (!createName.trim()) {
      errorMessage = "Informe um nome para o cofre.";
      return;
    }
    isSubmitting = true;
    errorMessage = null;
    try {
      await vaultStore.initVault(createPath.trim(), createName.trim());
      await canvasStore.loadWorkspace("default-workspace");
    } catch (err: any) {
      errorMessage = err?.message || String(err);
    } finally {
      isSubmitting = false;
    }
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget && vaultStore.isOpen) {
      vaultStore.closeModal();
    }
  }
</script>

{#if vaultStore.isModalOpen}
  <div class="modal-backdrop" onclick={handleBackdropClick} role="presentation">
    <div class="modal-container" role="dialog" aria-modal="true" aria-labelledby="modal-title">
      <!-- Modal Header -->
      <div class="modal-header">
        <div class="modal-title-wrap">
          <div class="vault-badge">
            <span class="vault-icon">🏛️</span>
          </div>
          <div>
            <h2 id="modal-title" class="modal-title">Gerenciar Cofre Local</h2>
            <p class="modal-subtitle">Fundação durável Sandland v2.0 (File-as-Truth & Anti-TOCTOU)</p>
          </div>
        </div>
        {#if vaultStore.isOpen}
          <button class="close-btn" onclick={() => vaultStore.closeModal()} title="Fechar">✕</button>
        {/if}
      </div>

      {#if !isTauri()}
        <div class="web-mode-notice">
          <span class="web-mode-icon">🌐</span>
          <div class="web-mode-body">
            <strong>Modo Navegador (Web Simulation Ativo)</strong>
            <p>
              Você está navegando em <code>localhost:1420</code>. O cofre e os eventos estão simulados localmente.
              Para usar o <strong>Rust Core nativo com gravação real em disco</strong>, inicie via desktop com: <code>npm run tauri dev</code>
            </p>
          </div>
        </div>
      {/if}

      <!-- Active Vault Status Card (if open) -->
      {#if vaultStore.isOpen}
        <div class="status-card">
          <div class="status-header">
            <span class="status-indicator"></span>
            <span class="status-title">Cofre Ativo: <strong>{vaultStore.vaultName}</strong></span>
            <span class="version-tag">Schema 2.0.0</span>
          </div>
          <div class="status-meta">
            <div class="meta-item">
              <span class="meta-label">Caminho:</span>
              <span class="meta-val mono" title={vaultStore.rootPath}>{vaultStore.rootPath}</span>
            </div>
            <div class="meta-metrics">
              <div class="metric-pill">
                <span class="metric-icon">📦</span>
                <span>Ativos CAS: <strong>{vaultStore.totalAssets}</strong></span>
              </div>
              <div class="metric-pill">
                <span class="metric-icon">📜</span>
                <span>Eventos Journal: <strong>{vaultStore.totalEvents}</strong></span>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Tabs -->
      <div class="tabs-nav">
        <button
          class="tab-btn"
          class:active={activeTab === "open"}
          onclick={() => { activeTab = "open"; errorMessage = null; }}
        >
          📂 Abrir Cofre Existente
        </button>
        <button
          class="tab-btn"
          class:active={activeTab === "create"}
          onclick={() => { activeTab = "create"; errorMessage = null; }}
        >
          ✨ Criar Novo Cofre
        </button>
      </div>

      <!-- Tab Content: Abrir Cofre -->
      {#if activeTab === "open"}
        <div class="tab-pane">
          <div class="form-group">
            <label for="open-path" class="form-label">Caminho do Diretório do Cofre</label>
            <div class="input-row">
              <input
                id="open-path"
                type="text"
                class="form-input mono"
                placeholder="Ex: C:\Users\nome\MeuCofre"
                bind:value={openPath}
                onkeydown={(e) => e.key === 'Enter' && handleOpenVault(openPath)}
              />
              <button
                class="primary-btn"
                disabled={isSubmitting || !openPath.trim()}
                onclick={() => handleOpenVault(openPath)}
              >
                {isSubmitting ? "Abrindo..." : "Abrir"}
              </button>
            </div>
          </div>

          <!-- Recent Vaults -->
          {#if vaultStore.recentVaults.length > 0}
            <div class="recent-section">
              <h4 class="recent-title">Cofres Recentes</h4>
              <div class="recent-list">
                {#each vaultStore.recentVaults as path}
                  <button
                    class="recent-item"
                    onclick={() => { openPath = path; handleOpenVault(path); }}
                    title={path}
                  >
                    <span class="recent-icon">📁</span>
                    <span class="recent-path mono">{path}</span>
                    <span class="recent-arrow">→</span>
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Tab Content: Criar Cofre -->
      {#if activeTab === "create"}
        <div class="tab-pane">
          <div class="form-group">
            <label for="create-name" class="form-label">Nome do Cofre</label>
            <input
              id="create-name"
              type="text"
              class="form-input"
              placeholder="Ex: Pesquisa de Mestrado"
              bind:value={createName}
            />
          </div>

          <div class="form-group">
            <label for="create-path" class="form-label">Caminho da Pasta</label>
            <input
              id="create-path"
              type="text"
              class="form-input mono"
              placeholder="Ex: C:\Users\nome\Documentos\SandlandVault"
              bind:value={createPath}
              onkeydown={(e) => e.key === 'Enter' && handleCreateVault()}
            />
            <p class="form-help">
              Uma estrutura canônica (vault.json, assets/, workspaces/, .history/) será inicializada.
            </p>
          </div>

          <div class="form-actions">
            <button
              class="primary-btn full-width"
              disabled={isSubmitting || !createPath.trim() || !createName.trim()}
              onclick={handleCreateVault}
            >
              {isSubmitting ? "Inicializando..." : "Criar e Abrir Cofre"}
            </button>
          </div>
        </div>
      {/if}

      <!-- Error alert -->
      {#if errorMessage}
        <div class="error-banner">
          <span class="error-icon">⚠️</span>
          <span class="error-text">{errorMessage}</span>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(5, 7, 10, 0.75);
    backdrop-filter: blur(6px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 16px;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .modal-container {
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-lg, 12px);
    width: 100%;
    max-width: 580px;
    box-shadow: 0 20px 45px rgba(0, 0, 0, 0.6);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 24px;
    color: var(--text-primary);
  }

  .modal-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
  }

  .modal-title-wrap {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .vault-badge {
    width: 40px;
    height: 40px;
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.2));
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md, 8px);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
  }

  .modal-title {
    font-size: 16px;
    font-weight: 600;
    margin: 0;
    color: var(--text-primary);
  }

  .modal-subtitle {
    font-size: 12px;
    color: var(--text-muted);
    margin: 2px 0 0 0;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 16px;
    cursor: pointer;
    padding: 4px;
    border-radius: var(--radius-sm);
    transition: color 0.15s;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .status-card {
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md, 8px);
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .status-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--accent-emerald, #10b981);
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.6);
  }

  .version-tag {
    margin-left: auto;
    font-size: 10px;
    padding: 2px 6px;
    background-color: rgba(6, 182, 212, 0.15);
    color: var(--accent-cyan, #06b6d4);
    border-radius: var(--radius-full, 9999px);
    font-weight: 600;
  }

  .status-meta {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .meta-item {
    font-size: 11px;
    color: var(--text-secondary);
    display: flex;
    gap: 6px;
    overflow: hidden;
  }

  .meta-label {
    color: var(--text-muted);
  }

  .meta-val {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-secondary);
  }

  .meta-metrics {
    display: flex;
    gap: 12px;
    margin-top: 4px;
  }

  .metric-pill {
    font-size: 11px;
    background-color: var(--bg-tertiary);
    border: 1px solid var(--border-subtle);
    padding: 3px 8px;
    border-radius: var(--radius-sm, 4px);
    display: flex;
    align-items: center;
    gap: 4px;
    color: var(--text-secondary);
  }

  .tabs-nav {
    display: flex;
    border-bottom: 1px solid var(--border-subtle);
    gap: 8px;
  }

  .tab-btn {
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    padding: 8px 12px;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    color: var(--text-primary);
  }

  .tab-btn.active {
    color: var(--accent-blue, #3b82f6);
    border-bottom-color: var(--accent-blue, #3b82f6);
  }

  .tab-pane {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .input-row {
    display: flex;
    gap: 8px;
  }

  .form-input {
    flex: 1;
    background-color: var(--bg-primary);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md, 8px);
    padding: 8px 12px;
    font-size: 13px;
    color: var(--text-primary);
    transition: border-color 0.15s;
  }

  .form-input:focus {
    outline: none;
    border-color: var(--border-focus, #5271ff);
  }

  .form-help {
    font-size: 11px;
    color: var(--text-muted);
    margin: 2px 0 0 0;
  }

  .primary-btn {
    background: linear-gradient(135deg, var(--accent-blue, #3b82f6), #2563eb);
    color: white;
    border: none;
    border-radius: var(--radius-md, 8px);
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.15s;
    white-space: nowrap;
  }

  .primary-btn:hover:not(:disabled) {
    opacity: 0.92;
  }

  .primary-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .primary-btn.full-width {
    width: 100%;
    padding: 10px;
  }

  .recent-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 6px;
  }

  .recent-title {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin: 0;
  }

  .recent-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 140px;
    overflow-y: auto;
  }

  .recent-item {
    background-color: var(--bg-tertiary);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm, 4px);
    padding: 6px 10px;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s ease;
  }

  .recent-item:hover {
    background-color: var(--bg-surface);
    border-color: var(--border-strong);
  }

  .recent-icon {
    font-size: 13px;
  }

  .recent-path {
    flex: 1;
    font-size: 11px;
    color: var(--text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .recent-arrow {
    font-size: 12px;
    color: var(--text-muted);
  }

  .web-mode-notice {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    background: rgba(59, 130, 246, 0.08);
    border: 1px solid rgba(59, 130, 246, 0.25);
    border-radius: var(--radius-md, 8px);
    padding: 10px 14px;
    font-size: 12px;
    color: var(--text-secondary);
  }

  .web-mode-icon {
    font-size: 18px;
    line-height: 1.2;
  }

  .web-mode-body strong {
    color: var(--accent-blue, #60a5fa);
    display: block;
    margin-bottom: 2px;
    font-size: 12px;
  }

  .web-mode-body p {
    margin: 0;
    line-height: 1.4;
    color: var(--text-secondary);
  }

  .web-mode-body code {
    background: rgba(0, 0, 0, 0.35);
    padding: 2px 6px;
    border-radius: 4px;
    color: #93c5fd;
    font-family: 'JetBrains Mono', monospace;
    font-size: 11px;
  }

  .error-banner {
    background-color: rgba(244, 63, 94, 0.12);
    border: 1px solid rgba(244, 63, 94, 0.35);
    color: var(--accent-rose, #f43f5e);
    border-radius: var(--radius-md, 8px);
    padding: 8px 12px;
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .mono {
    font-family: 'JetBrains Mono', monospace;
  }
</style>
