import { writable } from "svelte/store";

const internal = writable({
  type: "pending",
  display: getDisplayForFilter("pending"),
});

const filterTypes = ["today", "tomorrow", "pending", "yesterday"];

const todofilter = {
  subscribe: internal.subscribe,

  set: (type) => {
    if (filterTypes.indexOf(type) === -1) {
      throw `Filter ${type} is not supported.`;
    }

    internal.set({
      type,
      display: getDisplayForFilter(type),
    });
  },
};

function getDisplayForFilter(type) {
  if (type === "pending") {
    return "Pending todos";
  }
  return `todos for ${type.charAt(0).toUpperCase() + type.slice(1)}`;
}

export default todofilter;
