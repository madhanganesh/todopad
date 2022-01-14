<script>
  import TodoInput from "./TodoInput.svelte";
  import Todos from "./Todos.svelte";
  import Command from "./Command.svelte";

  let showCommand = false;
  let errorMessage = null;

  function onCommandHelp(e) {
    showCommand = e.detail;
  }

  function onError(e) {
    errorMessage = e.detail;
  }
</script>

<div class="todoarea">
  <TodoInput on:help={onCommandHelp} on:error={onError} />

  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {:else if showCommand}
    <Command />
  {:else}
    <Todos on:error={onError} />
  {/if}
</div>

<style>
  .todoarea {
    width: 90%;
    margin: 1.2rem auto;
  }

  .error {
    color: var(--red);
  }
</style>
