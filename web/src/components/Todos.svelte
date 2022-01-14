<script>
  import { onMount, createEventDispatcher } from "svelte";

  import auth from "../store/auth.js";
  import todos from "../store/todos.js";
  import todofilter from "../store/todofilter.js";
  import { getTodosAPI, updateTodoAPI, deleteTodoAPI } from "../helpers/api.js";

  const dispatch = createEventDispatcher();
  let focussedTodoID = null;

  onMount(async () => {
    try {
      const todoItems = await getTodosAPI($auth.authtoken, $todofilter.type);
      todos.set(todoItems);
    } catch (error) {
      dispatch("error", error);
    }
  });

  function onMouseOver(todoid) {
    focussedTodoID = todoid;
  }

  async function toggleTodoDone(todoid) {
    try {
      const todo = { ...$todos.find((t) => t.id === todoid) };
      todo.done = !todo.done;
      await updateTodoAPI($auth.authtoken, todo);
      const todoItems = await getTodosAPI($auth.authtoken, $todofilter.type);
      todos.set(todoItems);
    } catch (error) {
      dispatch("error", error);
    }
  }

  async function deleteTodo(todoid) {
    try {
      await deleteTodoAPI($auth.authtoken, todoid);
      const todoItems = await getTodosAPI($auth.authtoken, $todofilter.type);
      todos.set(todoItems);
    } catch (error) {
      dispatch("error", error);
    }
  }
</script>

<div class="todos">
  <p class="summary">
    {$todos.length !== 0 ? $todos.length : "No"}
    {$todofilter.display}
  </p>
  <ul>
    {#each $todos as todo (todo.id)}
      <li
        on:mouseleave={() => (focussedTodoID = null)}
        on:mouseover={() => onMouseOver(todo.id)}
        on:focus={() => onMouseOver(todo.id)}
      >
        <span class="completed" on:click={() => toggleTodoDone(todo.id)}>
          {#if !todo.done}
            <span class="fas fa-square" />
          {:else}
            <span class="fas fa-check-square" />
          {/if}
        </span>
        <span class="title">{todo.title}</span>
        {#if todo.id === focussedTodoID}
          <span class="controls">
            <!--span class="fas fa-edit" /-->
            <span class="fas fa-trash" on:click={() => deleteTodo(todo.id)} />
          </span>
        {/if}
      </li>
    {/each}
  </ul>
</div>

<style>
  .summary {
    font-size: 1.2rem;
    color: var(--magenta);
    display: flex;
  }

  .todos {
    margin-top: 1.2rem;
  }

  .todos ul {
    margin: 0;
    padding: 0;
    list-style-type: none;
  }

  .todos ul li {
    margin-bottom: 0.5rem;
    cursor: pointer;
    padding: 0.1rem;
    font-size: 1.5rem;
  }

  .todos ul li:hover {
    background-color: var(--plum);
  }

  .todos li .controls {
    float: right;
  }

  .todos li .controls [class*="fa-"] {
    margin-left: 10px;
  }

  @media (min-width: 850px) {
    .todos ul li {
      font-size: 1.3rem;
    }
  }
</style>
