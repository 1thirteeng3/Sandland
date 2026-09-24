<script lang="ts">
  import { pieceStore } from '../../stores/piece.svelte';

  let newPieceTitle = $state('');
  let isCreating = $state(false);

  async function handleCreatePiece() {
    const title = newPieceTitle.trim() || 'Nova Peça Editorial';
    isCreating = true;
    try {
      await pieceStore.createPiece(title);
      newPieceTitle = '';
    } catch (e) {
      console.error('Erro ao criar peça:', e);
    } finally {
      isCreating = false;
    }
  }

  function handleSelect(pieceId: string) {
    if (pieceStore.activePiece?.id !== pieceId) {
      pieceStore.selectPiece(pieceId);
    }
  }

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString('pt-BR', {
      day: '2-digit',
      month: 'short',
    });
  }
</script>

<aside class="piece-sidebar">
  <div class="sidebar-header">
    <div class="header-title">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
        <polyline points="14 2 14 8 20 8" />
        <line x1="16" y1="13" x2="8" y2="13" />
        <line x1="16" y1="17" x2="8" y2="17" />
        <polyline points="10 9 9 9 8 9" />
      </svg>
      <span>Peças Editoriais</span>
    </div>
    <span class="piece-count">{pieceStore.pieces.length}</span>
  </div>

  <div class="create-box">
    <input
      type="text"
      placeholder="Título da nova peça..."
      bind:value={newPieceTitle}
      onkeydown={(e) => e.key === 'Enter' && handleCreatePiece()}
    />
    <button
      class="create-btn"
      onclick={handleCreatePiece}
      disabled={isCreating}
      title="Criar Peça"
    >
      +
    </button>
  </div>

  <div class="pieces-list">
    {#if pieceStore.isLoading && pieceStore.pieces.length === 0}
      <div class="empty-state">Carregando peças...</div>
    {:else if pieceStore.pieces.length === 0}
      <div class="empty-state">Nenhuma peça editorial criada ainda.</div>
    {:else}
      {#each pieceStore.pieces as p (p.id)}
        {@const isActive = pieceStore.activePiece?.id === p.id}
        <button
          class="piece-item {isActive ? 'active' : ''}"
          onclick={() => handleSelect(p.id)}
        >
          <div class="item-main">
            <span class="item-title">{p.title || 'Sem Título'}</span>
            <div class="item-meta">
              <span class="word-count">{p.wordCount} palavras</span>
              {#if p.citationCount > 0}
                <span class="cit-count">
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
                  </svg>
                  {p.citationCount}
                </span>
              {/if}
              <span class="date">{formatDate(p.updatedAt)}</span>
            </div>
          </div>
        </button>
      {/each}
    {/if}
  </div>
</aside>

<style>
  .piece-sidebar {
    width: 240px;
    background: #12151b;
    border-right: 1px solid rgba(255, 255, 255, 0.08);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    user-select: none;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  }

  .header-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    color: #e2e8f0;
  }

  .piece-count {
    font-size: 0.72rem;
    background: rgba(255, 255, 255, 0.07);
    color: #94a3b8;
    padding: 1px 6px;
    border-radius: 10px;
  }

  .create-box {
    display: flex;
    gap: 6px;
    padding: 10px 14px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }

  .create-box input {
    flex: 1;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 6px;
    padding: 6px 10px;
    font-size: 0.78rem;
    color: #f1f5f9;
    outline: none;
    transition: border-color 0.15s;
  }

  .create-box input:focus {
    border-color: #6366f1;
  }

  .create-btn {
    background: #4f46e5;
    border: none;
    color: white;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s;
  }

  .create-btn:hover {
    background: #4338ca;
  }

  .pieces-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .empty-state {
    padding: 24px 12px;
    font-size: 0.75rem;
    color: #64748b;
    text-align: center;
  }

  .piece-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 8px 10px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s;
    color: inherit;
  }

  .piece-item:hover {
    background: rgba(255, 255, 255, 0.04);
  }

  .piece-item.active {
    background: rgba(99, 102, 241, 0.12);
    border-color: rgba(99, 102, 241, 0.35);
  }

  .item-main {
    flex: 1;
    min-width: 0;
  }

  .item-title {
    display: block;
    font-size: 0.82rem;
    font-weight: 500;
    color: #e2e8f0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .piece-item.active .item-title {
    color: #a5b4fc;
    font-weight: 600;
  }

  .item-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.7rem;
    color: #64748b;
    margin-top: 2px;
  }

  .cit-count {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    color: #818cf8;
  }
</style>
