import { writable } from "svelte/store";

const internal = writable({
  type: "pending",
  display: "Pending todos",
});

const todofilter = {
  subscribe: internal.subscribe,

  setPending: () => {
    internal.set({ type: "pending", display: "todos pending" });
  },

  setToday: () => {
    internal.set({
      type: "today",
      display: `todos for today (${new Date().toDateString()})`,
    });
  },
};

export default todofilter;
