<script>
  import { createEventDispatcher } from "svelte";

  import auth from "../store/auth.js";
  import todos from "../store/todos.js";
  import todofilter from "../store/todofilter.js";
  import { getTodosAPI, addTodoAPI } from "../helpers/api.js";

  const dispatch = createEventDispatcher();
  let title = "";

  function onTitleInput() {
    dispatch("help", title.startsWith("?"));
  }

  async function onCommand(command) {
    const type = command === "?" ? "pending" : command.slice(1);
    todofilter.set(type);
    const todoItems = await getTodosAPI($auth.authtoken, $todofilter.type);
    todos.set(todoItems);
  }

  async function onTitleChange(e) {
    dispatch("error", null);
    console.log(e.keyCode);
    if (title.length !== 0 && e.keyCode === 13) {
      e.preventDefault();
      try {
        if (title.startsWith("?")) {
          await onCommand(title.trim());
          return;
        }

        const todoItem = await addTodoAPI($auth.authtoken, {
          title,
          effort: 1.0,
          due: new Date(),
          done: false,
        });
        todos.add(todoItem);
      } catch (error) {
        dispatch("error", error);
      } finally {
        title = "";
        dispatch("help", false);
      }
    }
  }
  /*if (title.length !== 0) {
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

    if (errorMessage) {
      dispatch("error", errorMessage);
    }*/
</script>

<div class="todoinput">
  <input
    placeholder="enter a todo (or) type ?"
    type="text"
    bind:value={title}
    on:input={onTitleInput}
    on:keypress={onTitleChange}
    autofocus
  />
</div>

<style>
  .todoinput {
    margin-top: 1.2rem;
  }

  .todoinput input {
    width: 100%;
    height: 2.5rem;
    line-height: 2.5rem;
    font-family: var(--mono);
    font-size: 1rem;
    border: 2px solid var(--plum);
  }

  @media (min-width: 850px) {
    .todoinput input {
      height: 2.5rem;
      line-height: 3rem;
      font-size: 1.3rem;
    }
  }
</style>
