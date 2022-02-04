import dayjs from "dayjs";

export function movetags(todo) {
  if (!todo.title.includes("#")) {
    return todo;
  }

  let title = todo.title;
  const words = title.split(" ");
  let newwords = [];
  let tags = [];
  for (let word of words) {
    if (word.trim().startsWith("#")) {
      word = word.substring(1);
      tags.push(word);
    }
    newwords.push(word);
  }
  todo.title = newwords.join(" ");
  todo.tags = tags;
  return todo;
}

export function adjustForFields(todo) {
  let title = todo.title;
  const words = title.split(" ");
  const newwords = [];
  let tags = todo.tags;
  let due = todo.due;
  let effort = todo.effort;

  for (let word of words) {
    if (word.trim().startsWith("#")) {
      word = word.substring(1);
      tags.push(word);
      continue;
    }

    if (word.trim().startsWith(":")) {
      word = word.substring(1);
      if (word == "today") {
        due = new Date();
      }
      if (word == "tomorrow") {
        due = dayjs().add(1, "day").toDate();
      }
      if (word == "yesterday") {
        due = dayjs().subtract(1, "day").toDate();
      }
      if (dayjs(word).isValid()) {
        due = dayjs(word).toDate();
      }
      continue;
    }

    if (word.trim().startsWith("@")) {
      word = word.substring(1);
      const effortIs = parseFloat(word);
      if (effortIs != 0) {
        effort = effortIs;
      }
      continue;
    }

    newwords.push(word);
  }

  todo.title = newwords.join(" ");
  todo.tags = tags;
  todo.due = due;
  todo.effort = effort;

  return todo;
}
