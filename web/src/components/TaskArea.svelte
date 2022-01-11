<script>
  import { onMount } from "svelte";

  import auth from "../store/auth.js";
  import todos from "../store/todos.js";
  import todofilter from "../store/todofilter.js";

  import {
    getTodosAPI,
    addTodoAPI,
    updateTodoAPI,
    deleteTodoAPI,
  } from "../helpers/api.js";

  let title = "";
  let focussedTodoID = null;
  let errorMessage = null;
  let commands = [
    { key: "?pending", action: "show the pending todos" },
    { key: "?today", action: "show todos of today" },
    { key: "?yesterday", action: "show todos of yesterday" },
    { key: "?tomorrow", action: "show todos of tomorrow" },
    { key: "?this week", action: "show todos of this week" },
  ];

  onMount(async () => {
    try {
      const todoItems = await getTodosAPI($auth.authtoken, $todofilter.type);
      todos.set(todoItems);
    } catch (error) {
      errorMessage = error;
    }
  });

  async function onTitleChange(e) {
    errorMessage = null;
    if (e.keyCode === 13) {
      e.preventDefault();
      if (title.length !== 0) {
        if (title.trim() === "?today") {
          try {
            todofilter.setToday();
            const todoItems = await getTodosAPI(
              $auth.authtoken,
              $todofilter.type
            );
            todos.set(todoItems);
          } catch (error) {
            errorMessage = error;
          }
          title = "";
          return;
        }

        if (title.trim() === "?pending") {
          try {
            todofilter.setPending();
            const todoItems = await getTodosAPI(
              $auth.authtoken,
              $todofilter.type
            );
            todos.set(todoItems);
          } catch (error) {
            errorMessage = error;
          }
          title = "";
          return;
        }

        if (title.trim().startsWith("?")) {
          errorMessage = `command '${title}'' is not supported`;
          title = "";
          return;
        }

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
      const todoItems = await getTodosAPI($auth.authtoken, $todofilter.type);
      todos.set(todoItems);
    } catch (error) {
      errorMessage = error;
    }
  }

  async function deleteTodo(todoid) {
    try {
      await deleteTodoAPI($auth.authtoken, todoid);
      const todoItems = await getTodosAPI($auth.authtoken, $todofilter.type);
      todos.set(todoItems);
    } catch (error) {
      errorMessage = error;
    }
  }
</script>

<div class="todoarea">
  <div class="todoinput">
    <input
      placeholder="enter a todo (or) type ? to show possible commands"
      type="text"
      bind:value={title}
      on:keypress={onTitleChange}
      autofocus
    />
  </div>
  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}
  {#if title.startsWith("?")}
    <p class="summary">Type the command and press enter key for the action:</p>
    <ul class="command">
      {#each commands as command (command.key)}
        <li>
          <span class="command-key">{command.key}</span> -
          <span class="command-action">{command.action}</span>
        </li>
      {/each}
    </ul>
  {:else}
    <div class="todos">
      <p class="summary">{$todos.length} {$todofilter.display}</p>
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
                <span
                  class="fas fa-trash"
                  on:click={() => deleteTodo(todo.id)}
                />
              </span>
            {/if}
          </li>
        {/each}
      </ul>
    </div>
  {/if}
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
    height: 2.5rem;
    line-height: 2.5rem;
    font-family: var(--mono);
    font-size: 1.5rem;
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

  .error {
    color: var(--red);
  }

  .summary {
    font-size: 1.2rem;
    color: var(--magenta);
    display: flex;
  }

  ul.command {
    margin: 0;
    padding: 0;
  }

  ul.command li {
    list-style-type: none;
    color: var(--aqua);
    margin-bottom: 0.5rem;
    font-size: 1rem;
  }

  ul.command li .command-key {
    width: 6rem;
    display: inline-block;
  }

  ul.command li .command-action {
    display: inline-block;
    margin-left: 1rem;
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

    ul.command li {
      list-style-type: none;
      color: var(--aqua);
      margin-bottom: 0.5rem;
      font-size: 1.2rem;
    }

    ul.command li .command-key {
      width: 8rem;
      display: inline-block;
    }

    ul.command li .command-action {
      display: inline-block;
      margin-left: 1.2rem;
    }
  }
</style>
