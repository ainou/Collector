<script>
  import { createEventDispatcher } from "svelte";

  /** @type {boolean} */
  export let show = true;
  /** @type {boolean} */
  export let isLoading = false;
  /** @type {string} */
  export let saveToDailyShortcut = "";
  /** @type {string} */
  export let saveAsNoteShortcut = "";
  /** @type {string} */
  export let appendToNoteShortcut = "";

  const dispatch = createEventDispatcher();
</script>

{#if show}
  <div class="action-bar">
    <button
      class="action-btn"
      on:click={() => dispatch("daily")}
      disabled={isLoading}
      title={saveToDailyShortcut}
    >
      Daily Note
    </button>
    <button
      class="action-btn"
      on:click={() => dispatch("newNote")}
      disabled={isLoading}
      title={saveAsNoteShortcut}
    >
      New Note
    </button>
    <button
      class="action-btn"
      on:click={() => dispatch("append")}
      disabled={isLoading}
      title={appendToNoteShortcut}
    >
      Append
    </button>
  </div>
{/if}

<style>
  .action-bar {
    display: flex;
    gap: 6px;
    padding: 4px 12px 6px;
    flex-shrink: 0;
    justify-content: center;
  }

  .action-btn {
    padding: 5px 10px;
    border: 1px solid rgba(255, 255, 255, 0.16);
    border-radius: 999px;
    background: transparent;
    color: rgba(255, 255, 255, 0.72);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition:
      border-color 0.15s ease,
      color 0.15s ease,
      background-color 0.15s ease,
      opacity 0.15s ease;
    letter-spacing: 0.2px;
    line-height: 1;
  }

  .action-btn:hover:not(:disabled) {
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.28);
    background: rgba(255, 255, 255, 0.03);
  }

  .action-btn:active:not(:disabled) {
    background: rgba(255, 255, 255, 0.06);
    border-color: rgba(255, 255, 255, 0.32);
  }

  .action-btn:disabled {
    opacity: 0.45;
    cursor: default;
  }
</style>
