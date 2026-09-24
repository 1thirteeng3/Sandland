<script lang="ts">
  import PieceSidebar from './PieceSidebar.svelte';
  import PieceEditor from './PieceEditor.svelte';
</script>

<div class="editorial-view">
  <PieceSidebar />
  <PieceEditor />
</div>

<style>
  .editorial-view {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: #0f1217;
  }
</style>
