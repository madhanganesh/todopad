<script>
  import { onMount } from "svelte";
  import auth from "../store/auth.js";
  import todos from "../store/todos.js";
  import {
    getPendingTodosAPI,
    addTodoAPI,
    updateTodoAPI,
    deleteTodoAPI,
  } from "../helpers/api.js";

  let title = "";
  let focussedTodoID = null;
  let errorMessage = null;

  onMount(async () => {
    try {
      const todoItems = await getPendingTodosAPI($auth.authtoken);
      todos.set(todoItems);
    } catch (error) {
      errorMessage = error;
    }
  });

  async function onTitleChange(e) {
    if (e.keyCode === 13) {
      e.preventDefault();
      if (title.length !== 0) {
        try {
          const todoItem = await addTodoAPI($auth.authtoken, {
            title,
            effort: 1.0,
            due: new Date(),
            done: false,
          });
          todos.add(todoItem);
        } catch (error) {
          errorMessage = error;
        }
        title = "";
      }
    }
  }

  function onMouseOver(todoid) {
    focussedTodoID = todoid;
  }

  async function toggleTodoDone(todoid) {
    try {
      const todo = { ...$todos.find((t) => t.id === todoid) };
      todo.done = !todo.done;
      await updateTodoAPI($auth.authtoken, todo);
      //todos.toggleTodoDone(todoid);
      const todoItems = await getPendingTodosAPI($auth.authtoken);
      todos.set(todoItems);
    } catch (error) {
      errorMessage = error;
    }
  }

  async function deleteTodo(todoid) {
    try {
      await deleteTodoAPI($auth.authtoken, todoid);
      //todos.deleteTodo(todoid);
      const todoItems = await getPendingTodosAPI($auth.authtoken);
      todos.set(todoItems);
    } catch (error) {
      errorMessage = error;
    }
  }
</script>

<div class="todoarea">
  <div class="todoinput">
    <input
      placeholder="enter todo"
      type="text"
      bind:value={title}
      on:keypress={onTitleChange}
      autofocus
    />
  </div>
  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}
  <div class="todos">
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
</div>

<style>
  .todoarea {
    width: 90%;
    margin: 1.2rem auto;
  }

  .todoinput {
    margin-top: 1.2rem;
  }

  .todoinput input {
    width: 100%;
    height: 2rem;
    line-height: 2rem;
    font-family: var(--mono);
    font-size: 1rem;
    border: 2px solid var(--plum);
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
    margin-bottom: 0.3rem;
    cursor: pointer;
    padding: 0.1rem;
    font-size: 1rem;
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

  .error {
    color: var(--red);
  }

  @media (min-width: 850px) {
    .todoinput input {
      height: 2.5rem;
      line-height: 3rem;
      font-size: 1.3rem;
    }

    .todos ul li {
      font-size: 1.3rem;
    }
  }
</style>
