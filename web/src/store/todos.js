import { writable } from "svelte/store";

const todos = writable([]);

/*const todos = {
  subscribe: internal.subscribe,

  set: (todos) => {
    internal.set(todos);
  },

  add: (todo) => {
    internal.update((items) => {
      return [...items, todo];
    });
  },

  toggleTodoDone: (todoid) => {
    internal.update((items) => {
      let todo = { ...items.find((t) => t.id === todoid) };
      todo.done = !todo.done;
      const index = items.findIndex((t) => t.id == todoid);
      let updatedItems = [...items];
      updatedItems[index] = todo;
      return updatedItems;
    });
  },

  deleteTodo: (todoid) => {
    internal.update((items) => {
      const newItems = items.filter((t) => t.id !== todoid);
      return newItems;
    });
  },
};*/

export default todos;
