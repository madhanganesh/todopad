<script>
  import DisplayOptions from "./DisplayOptions.svelte";

  export let heading = "Todos";
  export let countSummary = "";
  let showFilters = false;
</script>

{#if showFilters}
  <DisplayOptions on:close={() => (showFilters = false)} />
{/if}

<p class="summary">
  <span class="filter">
    <a
      href="/"
      on:click|preventDefault|stopPropagation={() => (showFilters = true)}
    >
      {heading}
    </a>
  </span>

  <span class="count">
    {countSummary}
  </span>
</p>

<style>
  .summary {
    font-size: 1.2rem;
    display: flex;
    align-items: center;
    align-content: space-around;
  }

  .summary .filter a {
    color: var(--magenta);
    margin-right: 5px;
  }

  .summary .count {
    margin-left: auto;
  }
</style>
