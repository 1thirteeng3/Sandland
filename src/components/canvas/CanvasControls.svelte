<script lang="ts">
  import { canvasStore } from '../../stores/canvas.svelte';
  import { vaultService } from '../../services/vaultService';
  import { vaultStore } from '../../stores/vault.svelte';

  let fileInput: HTMLInputElement;

  function handleZoomIn() {
    const cx = window.innerWidth / 2;
    const cy = window.innerHeight / 2;
    canvasStore.zoomAt(1.2, cx, cy);
  }

  function handleZoomOut() {
    const cx = window.innerWidth / 2;
    const cy = window.innerHeight / 2;
    canvasStore.zoomAt(0.8, cx, cy);
  }

  function handleReset() {
    canvasStore.setViewport(0, 0, 1.0);
  }

  function handleAddCell() {
    const cx = (window.innerWidth / 2 - canvasStore.viewport.x) / canvasStore.viewport.zoom - 130;
    const cy = (window.innerHeight / 2 - canvasStore.viewport.y) / canvasStore.viewport.zoom - 75;
    const node = canvasStore.addNode('Nova Ideia', 'Clique duas vezes para editar esta célula.', cx, cy);
    canvasStore.startEditing(node.id);
  }

  function triggerAssetUpload() {
    if (!vaultStore.isOpen) {
      vaultStore.toggleModal();
      return;
    }
    fileInput?.click();
  }

  async function handleFileSelected(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);
      let binary = '';
      for (let i = 0; i < bytes.byteLength; i++) {
        binary += String.fromCharCode(bytes[i]);
      }
      const base64 = btoa(binary);
      const ext = file.name.split('.').pop() || 'bin';

      const assetRef = await vaultService.storeAsset(base64, ext);
      const cx = (window.innerWidth / 2 - canvasStore.viewport.x) / canvasStore.viewport.zoom - 140;
      const cy = (window.innerHeight / 2 - canvasStore.viewport.y) / canvasStore.viewport.zoom - 110;

      canvasStore.addAssetNode(file.name, assetRef, cx, cy);
    } catch (err) {
      console.error('[CanvasControls] Falha ao enviar anexo para o CAS:', err);
    } finally {
      input.value = '';
    }
  }

  let zoomPercentage = $derived(Math.round(canvasStore.viewport.zoom * 100));
</script>

<div class="canvas-controls">
  <button class="control-btn primary" onclick={handleAddCell} title="Adicionar Célula de Texto">+</button>
  <button class="control-btn" onclick={triggerAssetUpload} title="Adicionar Imagem/Anexo no CAS">🖼️</button>
  <input
    type="file"
    bind:this={fileInput}
    style="display: none;"
    accept="image/*,application/pdf"
    onchange={handleFileSelected}
  />
  <div class="divider"></div>
  <button class="control-btn" onclick={handleZoomIn} title="Aproximar (Zoom In)">🔍+</button>
  <span class="zoom-badge">{zoomPercentage}%</span>
  <button class="control-btn" onclick={handleZoomOut} title="Afastar (Zoom Out)">🔍-</button>
  <div class="divider"></div>
  <button class="control-btn" onclick={handleReset} title="Redefinir Câmera">🎯</button>
</div>

<style>
  .canvas-controls {
    position: absolute;
    bottom: 24px;
    right: 24px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: rgba(22, 25, 31, 0.85);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-full);
    box-shadow: var(--shadow-md, 0 4px 12px rgba(0, 0, 0, 0.3));
    z-index: 30;
  }

  .control-btn {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-full);
    background: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid transparent;
    font-size: 0.8125rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.15s;
  }

  .control-btn:hover {
    background: var(--border-strong);
    transform: scale(1.05);
  }

  .control-btn.primary {
    background: var(--accent-blue);
    color: #ffffff;
    font-weight: bold;
    font-size: 1.125rem;
  }

  .divider {
    width: 1px;
    height: 18px;
    background: var(--border-subtle);
  }

  .zoom-badge {
    font-size: 0.75rem;
    font-family: var(--font-mono, 'JetBrains Mono', monospace);
    color: var(--text-secondary);
    min-width: 42px;
    text-align: center;
  }
</style>
