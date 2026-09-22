<script lang="ts">
  import { onMount } from "svelte";
  import { ingestStore } from "./stores/ingest.svelte";
  import { canvasStore } from "./stores/canvas.svelte";
  import { vaultStore } from "./stores/vault.svelte";
  import IngestDrawer from "./components/ingest/IngestDrawer.svelte";
  import CanvasViewport from "./components/canvas/CanvasViewport.svelte";
  import VaultModal from "./components/vault/VaultModal.svelte";

  let isIngestOpen = $state(true);
  let activeWorkspaceTitle = $state("Mesa de Pesquisa Principal");

  onMount(async () => {
    try {
      await vaultStore.initFromEnvironment();
      await Promise.allSettled([
        ingestStore.loadItems(),
        canvasStore.loadWorkspace("default-workspace"),
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
    <CanvasViewport />
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
</style>
