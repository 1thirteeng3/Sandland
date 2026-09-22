<script lang="ts">
  import { onMount } from "svelte";
  import { ingestStore } from "./stores/ingest.svelte";
  import { canvasStore } from "./stores/canvas.svelte";
  import IngestDrawer from "./components/ingest/IngestDrawer.svelte";
  import CanvasViewport from "./components/canvas/CanvasViewport.svelte";

  let isIngestOpen = $state(true);
  let activeVaultName = $state("Cofre Sandland");
  let activeWorkspaceTitle = $state("Mesa de Pesquisa Principal");

  onMount(async () => {
    try {
      await Promise.allSettled([
        ingestStore.loadItems(),
        canvasStore.loadWorkspace("default-workspace"),
      ]);
    } catch (err) {
      console.error("[App] Falha na hidratação a frio:", err);
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
      <span class="vault-name">{activeVaultName}</span>
      <span class="workspace-pill">{activeWorkspaceTitle}</span>
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

  .vault-name {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .workspace-pill {
    font-size: 12px;
    padding: 3px 10px;
    background-color: var(--bg-tertiary);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-full);
    color: var(--accent-cyan);
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
