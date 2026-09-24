<script lang="ts">
  import type { CitationRecordDTO } from '../../types/piece';
  import { pieceStore } from '../../stores/piece.svelte';
  import { canvasStore } from '../../stores/canvas.svelte';

  interface Props {
    citation: CitationRecordDTO;
  }

  let { citation }: Props = $props();

  function handleLocateOnBoard(e: MouseEvent) {
    e.stopPropagation();
    // Centraliza o canvas no nó citado
    const node = canvasStore.nodes.find((n) => n.id === citation.sourceCellId);
    if (node) {
      canvasStore.viewport = {
        x: -node.x + window.innerWidth / 4,
        y: -node.y + window.innerHeight / 4,
        zoom: 1.0,
      };
      canvasStore.selectedNodeId = node.id;
    }
    // Se não estiver em split-view, alterna para split para visualizar na Mesa
    if (pieceStore.viewMode === 'piece') {
      pieceStore.setViewMode('split');
    }
  }
</script>

<div
  class="citation-badge"
  role="button"
  tabindex="0"
  onclick={handleLocateOnBoard}
  onkeydown={(e) => e.key === 'Enter' && handleLocateOnBoard(e as any)}
  title="Origem: {citation.sourceTitle} (v{citation.sourceRevision}) • Clique para ver na Mesa"
>
  <span class="badge-icon">📋</span>
  <span class="badge-title">{citation.sourceTitle}</span>
  <span class="badge-rev">v{citation.sourceRevision}</span>

  <button
    class="locate-btn"
    onclick={handleLocateOnBoard}
    title="Localizar cartão na Mesa Espacial"
  >
    <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10" />
      <polygon points="12 8 8 12 12 16 12 8" />
    </svg>
  </button>
</div>

<style>
  .citation-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px;
    border-radius: 6px;
    font-size: 0.76rem;
    font-weight: 500;
    cursor: pointer;
    user-select: none;
    transition: all 0.15s ease;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(26, 30, 38, 0.85);
    backdrop-filter: blur(8px);
    color: #e2e8f0;
    max-width: 100%;
  }

  .citation-badge:hover {
    border-color: rgba(99, 102, 241, 0.5);
    background: rgba(36, 42, 54, 0.95);
    transform: translateY(-1px);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.25);
  }

  .badge-icon {
    font-size: 0.72rem;
    opacity: 0.8;
  }

  .badge-title {
    max-width: 160px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: #cbd5e1;
  }

  .badge-rev {
    color: #64748b;
    font-size: 0.7rem;
    font-family: monospace;
  }

  .locate-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    padding: 2px;
    margin-left: 2px;
    color: #94a3b8;
    cursor: pointer;
    border-radius: 4px;
    transition: color 0.15s, background-color 0.15s;
  }

  .locate-btn:hover {
    color: #38bdf8;
    background: rgba(56, 189, 248, 0.15);
  }
</style>
