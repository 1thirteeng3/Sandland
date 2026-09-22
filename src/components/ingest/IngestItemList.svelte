<script lang="ts">
  import { ingestStore } from '../../stores/ingest.svelte';
  import type { IngestedItemDTO } from '../../types/ingest';

  let searchQuery = $state('');

  function handleSearch() {
    ingestStore.setFilter({ query: searchQuery.trim() || undefined });
  }

  function getBadgeClass(state: IngestedItemDTO['state']) {
    switch (state) {
      case 'Classified':
        return 'badge-emerald';
      case 'Classifying':
        return 'badge-purple';
      case 'Extracting':
        return 'badge-blue';
      case 'NeedsManualReview':
        return 'badge-rose';
      case 'Failed':
        return 'badge-rose';
      default:
        return 'badge-amber';
    }
  }

  function formatDate(timestamp: number) {
    return new Date(timestamp * 1000).toLocaleDateString('pt-BR', {
      day: '2-digit',
      month: 'short',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  function sanitizeDisplaySnippet(snippet?: string): string {
    if (!snippet) return '';
    return snippet
      .replace(/^#+\s+/gm, '')
      .replace(/&nbsp;/g, ' ')
      .replace(/&quot;/g, '"')
      .replace(/&amp;/g, '&')
      .replace(/&lt;/g, '<')
      .replace(/&gt;/g, '>')
      .replace(/&#39;/g, "'")
      .replace(/\s+/g, ' ')
      .trim();
  }
</script>

<div class="item-list-container">
  <div class="search-bar">
    <input
      type="text"
      placeholder="Buscar notas e documentos..."
      bind:value={searchQuery}
      oninput={handleSearch}
    />
  </div>

  <div class="list-scroll">
    {#if ingestStore.isLoading && ingestStore.items.length === 0}
      <div class="empty-state">Carregando acervo...</div>
    {:else if ingestStore.items.length === 0}
      <div class="empty-state">
        <p class="empty-title">Nenhum documento encontrado</p>
        <p class="empty-desc">Arraste arquivos Markdown para começar a alimentar o cofre.</p>
      </div>
    {:else}
      {#each ingestStore.items as item (item.id)}
        <div
          class="item-card"
          class:selected={ingestStore.selectedItem?.id === item.id}
          onclick={() => (ingestStore.selectedItem = item)}
          role="button"
          tabindex="0"
          draggable="true"
          ondragstart={(e) => {
            if (e.dataTransfer) {
              e.dataTransfer.setData('application/json', JSON.stringify({
                id: item.id,
                title: item.title,
                content: item.contentSnippet || item.title,
              }));
              e.dataTransfer.effectAllowed = 'copy';
            }
          }}
          onkeydown={(e) => e.key === 'Enter' && (ingestStore.selectedItem = item)}
        >
          <div class="card-header">
            <span class="card-title" title={item.title}>{item.title}</span>
            <span class="state-badge {getBadgeClass(item.state)}">{item.state}</span>
          </div>

          <p class="card-snippet">{sanitizeDisplaySnippet(item.contentSnippet)}</p>

          <div class="card-footer">
            <div class="tags-group">
              {#if item.category}
                <span class="category-chip">📁 {item.category}</span>
              {/if}
              {#each item.tags.slice(0, 3) as tag}
                <span class="tag-chip">#{tag}</span>
              {/each}
              {#if item.tags.length > 3}
                <span class="tag-more">+{item.tags.length - 3}</span>
              {/if}
            </div>
            <span class="timestamp">{formatDate(item.createdAt)}</span>
          </div>

          {#if item.needsManualReview}
            <div class="review-indicator">
              ⚠️ Requer revisão humana de taxonomia
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .item-list-container {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
    gap: 10px;
  }

  .search-bar input {
    width: 100%;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-sm);
    font-size: 0.8125rem;
    color: var(--text-primary);
    transition: border-color 0.2s;
  }

  .search-bar input:focus {
    border-color: var(--accent-blue);
  }

  .list-scroll {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-right: 4px;
  }

  .item-card {
    padding: 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all 0.15s ease-in-out;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .item-card:hover {
    border-color: var(--border-strong);
    background: var(--bg-tertiary);
  }

  .item-card.selected {
    border-color: var(--accent-blue);
    background: rgba(59, 130, 246, 0.08);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
  }

  .card-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .state-badge {
    font-size: 0.6875rem;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .badge-amber {
    background: rgba(245, 158, 11, 0.15);
    color: var(--accent-amber);
  }

  .badge-blue {
    background: rgba(59, 130, 246, 0.15);
    color: var(--accent-blue);
  }

  .badge-purple {
    background: rgba(139, 92, 246, 0.15);
    color: var(--accent-purple);
  }

  .badge-emerald {
    background: rgba(16, 185, 129, 0.15);
    color: var(--accent-emerald);
  }

  .badge-rose {
    background: rgba(244, 63, 94, 0.15);
    color: var(--accent-rose);
  }

  .card-snippet {
    font-size: 0.75rem;
    color: var(--text-secondary);
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 4px;
  }

  .tags-group {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
    align-items: center;
  }

  .category-chip {
    font-size: 0.6875rem;
    padding: 1px 5px;
    background: var(--bg-surface);
    color: var(--text-primary);
    border-radius: var(--radius-sm);
  }

  .tag-chip {
    font-size: 0.6875rem;
    padding: 1px 5px;
    background: rgba(6, 182, 212, 0.1);
    color: var(--accent-cyan);
    border-radius: var(--radius-sm);
  }

  .tag-more {
    font-size: 0.6875rem;
    color: var(--text-muted);
  }

  .timestamp {
    font-size: 0.6875rem;
    color: var(--text-muted);
  }

  .review-indicator {
    font-size: 0.6875rem;
    color: var(--accent-amber);
    background: rgba(245, 158, 11, 0.08);
    padding: 3px 6px;
    border-radius: var(--radius-sm);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 16px;
    text-align: center;
    color: var(--text-muted);
  }

  .empty-title {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .empty-desc {
    font-size: 0.75rem;
  }
</style>
