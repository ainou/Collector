<script>
  import { createEventDispatcher } from "svelte";
  import WikilinkPicker from "../WikilinkPicker.svelte";

  /** @type {boolean} */
  export let open = false;
  /** @type {Array} */
  export let matches = [];
  /** @type {number} */
  export let selectedIndex = 0;
  /** @type {{ left: number, top: number }} */
  export let position = { left: 0, top: 0 };
  /** @type {boolean} */
  export let showPaths = true;

  const dispatch = createEventDispatcher();

  /** @type {(index: number) => void} */
  export let onHover = () => {};

  function handleSelect(note) {
    dispatch("select", note);
  }
</script>

{#if open && matches.length > 0}
  <div
    class="wikilink-picker-position"
    style="left: {position.left}px; top: {position.top}px;"
  >
    <WikilinkPicker
      notes={matches}
      {selectedIndex}
      {showPaths}
      onSelect={handleSelect}
      {onHover}
    />
  </div>
{/if}

<style>
  .wikilink-picker-position {
    position: fixed;
    z-index: 200;
    min-width: 220px;
    max-width: 320px;
  }
</style>
