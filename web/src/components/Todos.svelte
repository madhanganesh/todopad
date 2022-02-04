<script>
  import dayjs from "dayjs";
  import extractUrls from "extract-urls";

  import { onDestroy, createEventDispatcher } from "svelte";
  import { slide } from "svelte/transition";

  import auth from "../store/auth.js";
  import todos from "../store/todos.js";
  import displayoption from "../store/displayoption.js";
  import { getTodosAPI, updateTodoAPI, deleteTodoAPI } from "../helpers/api.js";

  import Summary from "./Summary.svelte";
  import EditTodo from "./EditTodo.svelte";

  const dispatch = createEventDispatcher();
  let focussedTodoID = null;
  let selectedTodo = null;

  async function loadTodos(filter) {
    try {
      const todoItems = await getTodosAPI($auth.authtoken, filter);
      todos.set(todoItems);
    } catch (error) {
      dispatch("error", error);
    }
  }

  const unsubscribe = displayoption.subscribe((d) => {
    if (d.option === "todos") {
      loadTodos(d.todosfilter);
    }
  });

  onDestroy(() => unsubscribe());

  function onMouseOver(todoid) {
    focussedTodoID = todoid;
  }

  async function toggleTodoDone(todoid) {
    try {
      const todo = { ...$todos.find((t) => t.id === todoid) };
      todo.done = !todo.done;
      await updateTodoAPI($auth.authtoken, todo);
      //const todoItems = await getTodosAPI($auth.authtoken, $todofilter.value);
      //todos.set(todoItems);
      loadTodos($displayoption.todosfilter);
    } catch (error) {
      dispatch("error", error);
    }
  }

  async function deleteTodo(todoid) {
    try {
      await deleteTodoAPI($auth.authtoken, todoid);
      //const todoItems = await getTodosAPI($auth.authtoken, $todofilter.value);
      //todos.set(todoItems);
      loadTodos($displayoption.todosfilter);
    } catch (error) {
      dispatch("error", error);
    }
  }

  function showTodoDetail(todo) {
    selectedTodo = todo;
  }

  function onSaveTodo() {
    loadTodos($displayoption.todosfilter);
    selectedTodo = null;
  }

  function onCancelEdit() {
    selectedTodo = null;
  }

  function getUrls(notes) {
    const urls = extractUrls(notes);
    if (!urls) return [];
    return urls;
  }

  function countSummary(count) {
    if (count === 0) {
      return "Nothing to do";
    }

    if (count === 1) {
      return `${count} todo`;
    }

    return `${count} todos`;
  }

  async function onEffort(todoid) {
    try {
      const todo = { ...$todos.find((t) => t.id === todoid) };
      todo.effort += 0.5;
      await updateTodoAPI($auth.authtoken, todo);
      loadTodos($displayoption.todosfilter);
    } catch (error) {
      console.error(error);
    }
  }

  async function onDue(todoid) {
    try {
      const todo = { ...$todos.find((t) => t.id === todoid) };
      todo.due = dayjs(todo.due).add(1, "d").toDate();
      await updateTodoAPI($auth.authtoken, todo);
      loadTodos($displayoption.todosfilter);
    } catch (error) {
      console.error(error);
    }
  }
</script>

{#if selectedTodo}
  <EditTodo
    id={selectedTodo.id}
    on:save={onSaveTodo}
    on:cancel={onCancelEdit}
  />
{/if}

<div class="todos">
  <Summary
    heading={$displayoption.todosfilter.heading}
    countSummary={countSummary($todos.length)}
  />
  <ul>
    {#each $todos as todo (todo.id)}
      <li
        transition:slide
        on:click|preventDefault={showTodoDetail(todo)}
        on:mouseleave={() => (focussedTodoID = null)}
        on:mouseover={() => onMouseOver(todo.id)}
        on:focus={() => onMouseOver(todo.id)}
      >
        <span
          class="completed"
          on:click|preventDefault|stopPropagation={() =>
            toggleTodoDone(todo.id)}
        >
          {#if !todo.done}
            <span class="fas fa-square" />
          {:else}
            <span class="fas fa-check-square" />
          {/if}
        </span>
        <span class="title">{todo.title}</span>
        {#if todo.id === focussedTodoID}
          <span class="controls">
            <span
              class="effort"
              title="click to add 0.5"
              on:click|preventDefault|stopPropagation={() => onEffort(todo.id)}
              >{todo.effort} hr</span
            >
            <span
              class="due"
              title="clcick to move next day"
              on:click|preventDefault|stopPropagation={() => onDue(todo.id)}
              >{dayjs(todo.due).format("DD MMM")}</span
            >
            <!--span class="fas fa-edit" /-->
            {#each getUrls(todo.notes) as link}
              <span
                class="fas fa-link"
                title={link}
                on:click|preventDefault|stopPropagation={() =>
                  window.open(link)}
              />
            {/each}
            <span
              class="fas fa-trash"
              on:click|preventDefault|stopPropagation={() =>
                deleteTodo(todo.id)}
            />
          </span>
        {/if}
      </li>
    {/each}
  </ul>
</div>

<style>
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

  .controls .effort,
  .controls .due {
    font-size: 90%;
    color: var(--magenta);
    margin-left: 10px;
  }

  @media (min-width: 850px) {
    .todos ul li {
      font-size: 1.3rem;
    }
  }
</style>
