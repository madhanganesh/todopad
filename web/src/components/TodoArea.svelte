<script>
  import auth from "../store/auth.js";
  import todos from "../store/todos.js";
  import displayoption from "../store/displayoption.js";
  import { getTodosAPI } from "../helpers/api.js";

  import TodoInput from "./TodoInput.svelte";
  import Todos from "./Todos.svelte";
  import ShowReport from "./Report.svelte";

  let errorMessage = null;

  function onError(e) {
    if (e.detail) {
      console.error(e.detail);
      errorMessage = e.detail;
    }
  }

  async function loadTodos(filter) {
    try {
      const todoItems = await getTodosAPI($auth.authtoken, filter);
      todos.set(todoItems);
    } catch (error) {
      errorMessage = error;
    }
  }

  async function onTodosChanged() {
    if ($displayoption.option === "todos") {
      loadTodos($displayoption.todosfilter);
    }
  }
</script>

<div class="todoarea">
  <TodoInput on:error={onError} on:change={onTodosChanged} />

  {#if errorMessage}
    <p class="error">
      <span class="message">{errorMessage}</span>
      <span
        id="close"
        class="fas fa-times-circle"
        on:click|preventDefault|stopPropagation={() =>
          (errorMessage = undefined)}
      />
    </p>
  {/if}

  {#if !errorMessage}
    {#if $displayoption.option === "todos"}
      <Todos on:error={onError} />
    {/if}
    {#if $displayoption.option === "report"}
      <ShowReport />
    {/if}
  {/if}
</div>

<style>
  .todoarea {
    width: 90%;
    margin: 1.2rem auto;
  }

  .error {
    display: flex;
  }

  .error .message {
    color: var(--red);
  }

  .error #close {
    margin-left: auto;
    cursor: pointer;
  }
</style>
