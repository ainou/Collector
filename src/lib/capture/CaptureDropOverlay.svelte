<script>
  /** @type {boolean} */
  export let show = false;
</script>

{#if show}
  <div class="drop-overlay"></div>
{/if}

<style>
  .drop-overlay {
    position: absolute;
    inset: 2px;
    background: none;
    border: 2px dashed rgba(255, 255, 255, 0.7);
    border: 2px rgba(255, 255, 255, 0.7);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    pointer-events: none;
  }
</style>
