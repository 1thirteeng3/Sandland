<script lang="ts">
  import { onMount } from "svelte";
  import { ingestStore } from "./stores/ingest.svelte";
  import { canvasStore } from "./stores/canvas.svelte";
  import { vaultStore } from "./stores/vault.svelte";
  import { pieceStore } from "./stores/piece.svelte";
  import IngestDrawer from "./components/ingest/IngestDrawer.svelte";
  import CanvasViewport from "./components/canvas/CanvasViewport.svelte";
  import EditorialView from "./components/editorial/EditorialView.svelte";
  import VaultModal from "./components/vault/VaultModal.svelte";

  let isIngestOpen = $state(false);
  let activeWorkspaceTitle = $state("Mesa de Pesquisa Principal");

  onMount(async () => {
    try {
      await vaultStore.initFromEnvironment();
      await Promise.allSettled([
        ingestStore.loadItems(),
        canvasStore.loadWorkspace("default-workspace"),
        pieceStore.loadWorkspace("default-workspace"),
      ]);
    } catch (err) {
      console.error("[App] Falha na hidratação inicial:", err);
    }
  });

  function toggleIngest() {
    isIngestOpen = !isIngestOpen;
  }
</script>

<main class="app-layout">
  <!-- Top Navigation & Workspace Header -->
  <header class="app-header">
    <div class="header-left">
      <div class="logo">
        <span class="logo-badge">S</span>
        <span class="app-title">SANDLAND</span>
      </div>
      <div class="divider"></div>

      <!-- Interactive Vault Selector Button -->
      <button 
        class="vault-selector-btn" 
        onclick={() => vaultStore.toggleModal()}
        title="Gerenciar ou Alternar Cofre Local"
      >
        <span class="vault-icon">🏛️</span>
        <span class="vault-name">{vaultStore.vaultName}</span>
        <span class="selector-arrow">▾</span>
      </button>

      <span class="workspace-pill">{activeWorkspaceTitle}</span>

      <!-- Sync / Journal Status Indicator -->
      <div class="sync-indicator sync-{vaultStore.syncState}" title="Status do Journal e Persistência File-as-Truth">
        <span class="sync-dot"></span>
        <span class="sync-label">
          {#if vaultStore.syncState === 'synced'}
            Gravado
          {:else if vaultStore.syncState === 'saving'}
            Journaling...
          {:else if vaultStore.syncState === 'conflict'}
            Conflito OCC
          {:else if vaultStore.syncState === 'error'}
            Erro de I/O
          {:else}
            Offline
          {/if}
        </span>
      </div>
    </div>

    <!-- View Mode Switcher: Mesa / Split-View / Peça -->
    <div class="header-center">
      <div class="view-mode-group">
        <button
          class="view-mode-btn {pieceStore.viewMode === 'board' ? 'active' : ''}"
          onclick={() => pieceStore.setViewMode('board')}
          title="Modo Mesa Espacial (PixiJS Canvas)"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <path d="M3 9h18M9 21V9" />
          </svg>
          <span>Mesa</span>
        </button>

        <button
          class="view-mode-btn {pieceStore.viewMode === 'split' ? 'active' : ''}"
          onclick={() => pieceStore.setViewMode('split')}
          title="Modo Dividido: Mesa à Esquerda, Peça à Direita (Fork-on-Insert)"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <line x1="12" y1="3" x2="12" y2="21" />
          </svg>
          <span>Split-View</span>
        </button>

        <button
          class="view-mode-btn {pieceStore.viewMode === 'piece' ? 'active' : ''}"
          onclick={() => pieceStore.setViewMode('piece')}
          title="Modo Peça Editorial (Documento Longo)"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <polyline points="14 2 14 8 20 8" />
            <line x1="16" y1="13" x2="8" y2="13" />
            <line x1="16" y1="17" x2="8" y2="17" />
          </svg>
          <span>Peça</span>
        </button>
      </div>
    </div>

    <div class="header-right">
      <button 
        class="action-btn" 
        class:active={isIngestOpen} 
        onclick={toggleIngest} 
        title="Alternar Gaveta de Ingestão"
      >
        <span class="btn-icon">📥</span>
        <span>Ingestão</span>
      </button>
    </div>
  </header>

  <!-- Central Workspace & Canvas Area -->
  <div class="workspace-body">
    {#if pieceStore.viewMode === 'board'}
      <CanvasViewport />
    {:else if pieceStore.viewMode === 'piece'}
      <EditorialView />
    {:else if pieceStore.viewMode === 'split'}
      <div class="split-view-container">
        <div class="split-pane canvas-pane">
          <CanvasViewport />
        </div>
        <div class="split-pane editorial-pane">
          <EditorialView />
        </div>
      </div>
    {/if}

    <IngestDrawer bind:isOpen={isIngestOpen} onClose={() => (isIngestOpen = false)} />
  </div>

  <!-- Vault Management Modal -->
  <VaultModal />
</main>

<style>
  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background-color: var(--bg-primary);
    overflow: hidden;
  }

  .app-header {
    height: 48px;
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    z-index: 50;
  }

  .header-left, .header-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .logo-badge {
    background: linear-gradient(135deg, var(--accent-blue), var(--accent-purple));
    color: white;
    font-weight: 700;
    font-size: 13px;
    width: 24px;
    height: 24px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .app-title {
    font-size: 13px;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--text-primary);
  }

  .divider {
    width: 1px;
    height: 18px;
    background-color: var(--border-subtle);
  }

  .vault-selector-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: 1px solid transparent;
    padding: 4px 8px;
    border-radius: var(--radius-sm, 4px);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .vault-selector-btn:hover {
    background-color: var(--bg-tertiary);
    border-color: var(--border-subtle);
  }

  .vault-icon {
    font-size: 14px;
  }

  .vault-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    max-width: 180px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .selector-arrow {
    font-size: 10px;
    color: var(--text-muted);
  }

  .workspace-pill {
    font-size: 12px;
    padding: 3px 10px;
    background-color: var(--bg-tertiary);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-full);
    color: var(--accent-cyan);
  }

  .sync-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    padding: 2px 8px;
    border-radius: var(--radius-full, 9999px);
    background-color: var(--bg-tertiary);
    border: 1px solid var(--border-subtle);
    color: var(--text-secondary);
  }

  .sync-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .sync-synced .sync-dot {
    background-color: var(--accent-emerald, #10b981);
    box-shadow: 0 0 6px rgba(16, 185, 129, 0.6);
  }

  .sync-saving .sync-dot {
    background-color: var(--accent-amber, #f59e0b);
    animation: pulse 1s infinite alternate;
  }

  .sync-conflict .sync-dot {
    background-color: var(--accent-amber, #f59e0b);
  }

  .sync-error .sync-dot {
    background-color: var(--accent-rose, #f43f5e);
    box-shadow: 0 0 6px rgba(244, 63, 94, 0.6);
  }

  @keyframes pulse {
    from { opacity: 0.4; }
    to { opacity: 1; }
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    font-size: 12px;
    font-weight: 500;
    border-radius: var(--radius-md);
    background-color: var(--bg-surface);
    color: var(--text-secondary);
    border: 1px solid var(--border-subtle);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn:hover {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    border-color: var(--border-strong);
  }

  .action-btn.active {
    background-color: rgba(59, 130, 246, 0.15);
    color: var(--accent-blue);
    border-color: var(--accent-blue);
  }

  .workspace-body {
    flex: 1;
    position: relative;
    display: flex;
    overflow: hidden;
  }

  .header-center {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .view-mode-group {
    display: flex;
    align-items: center;
    background: rgba(15, 18, 24, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: var(--radius-md, 8px);
    padding: 2px;
    gap: 2px;
  }

  .view-mode-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    color: #94a3b8;
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .view-mode-btn:hover {
    color: #f1f5f9;
    background: rgba(255, 255, 255, 0.04);
  }

  .view-mode-btn.active {
    background: #1e2430;
    color: #60a5fa;
    border-color: rgba(96, 165, 250, 0.3);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .drift-header-indicator {
    font-size: 0.7rem;
    line-height: 1;
  }

  .split-view-container {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .split-pane {
    height: 100%;
    overflow: hidden;
    position: relative;
  }

  .split-pane.canvas-pane {
    flex: 1;
    min-width: 320px;
    border-right: 1px solid rgba(255, 255, 255, 0.08);
  }

  .split-pane.editorial-pane {
    flex: 1.1;
    min-width: 400px;
  }
</style>
