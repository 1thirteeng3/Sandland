<script lang="ts">
  import { onMount } from 'svelte';
  import { ingestStore } from '../../stores/ingest.svelte';
  import Dropzone from './Dropzone.svelte';
  import IngestItemList from './IngestItemList.svelte';

  let { isOpen = $bindable(true), onClose }: { isOpen?: boolean; onClose?: () => void } = $props();

  onMount(() => {
    ingestStore.loadItems();
  });
</script>

<aside class="ingest-drawer" class:collapsed={!isOpen}>
  <header class="drawer-header">
    <div class="header-info">
      <h2 class="title">Acervo & Ingestão</h2>
      <div class="counts">
        <span class="count-pill">{ingestStore.totalCount} itens</span>
        {#if ingestStore.needsReviewCount > 0}
          <span class="count-pill review">{ingestStore.needsReviewCount} para revisar</span>
        {/if}
      </div>
    </div>
    <button class="toggle-btn" onclick={() => (isOpen = !isOpen)} aria-label="Alternar gaveta">
      {#if isOpen}
        ◀
      {:else}
        ▶
      {/if}
    </button>
  </header>

  {#if isOpen}
    <div class="drawer-content">
      <Dropzone />
      <hr class="divider" />
      <IngestItemList />
    </div>
  {/if}
</aside>

<style>
  .ingest-drawer {
    width: 360px;
    height: 100%;
    background: rgba(13, 15, 18, 0.85);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border-right: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    transition: width 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 20;
    position: relative;
  }

  .ingest-drawer.collapsed {
    width: 48px;
  }

  .drawer-header {
    padding: 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border-subtle);
    min-height: 64px;
  }

  .header-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    overflow: hidden;
  }

  .collapsed .header-info {
    display: none;
  }

  .title {
    font-size: 0.9375rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .counts {
    display: flex;
    gap: 6px;
  }

  .count-pill {
    font-size: 0.6875rem;
    padding: 1px 6px;
    border-radius: var(--radius-full);
    background: var(--bg-surface);
    color: var(--text-secondary);
  }

  .count-pill.review {
    background: rgba(245, 158, 11, 0.15);
    color: var(--accent-amber);
    font-weight: 600;
  }

  .toggle-btn {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-sm);
    background: var(--bg-secondary);
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    transition: all 0.15s;
  }

  .toggle-btn:hover {
    background: var(--bg-surface);
    color: var(--text-primary);
  }

  .drawer-content {
    padding: 16px;
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
    gap: 16px;
  }

  .divider {
    border: none;
    border-top: 1px solid var(--border-subtle);
  }
</style>
