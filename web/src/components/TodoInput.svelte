<script>
  import { createEventDispatcher } from "svelte";

  import auth from "../store/auth.js";
  import displayoption from "../store/displayoption.js";
  import { addTodoAPI } from "../helpers/api.js";
  import { adjustForFields } from "../helpers/utils.js";
  import { getUserTags } from "../helpers/api.js";

  const dispatch = createEventDispatcher();
  let title = "";

  let showHint = undefined;
  let usertags;
  let intags = false;

  async function getUserTagss() {
    try {
      const tagsres = await getUserTags($auth.authtoken);
      usertags = tagsres.tags;
    } catch (e) {
      console.error(e);
      usertags = "";
    }
  }

  function getTagsHint(title, allTags) {
    let matches = title.matchAll(/#([a-zA-Z0-9]+)/g);

    let titleTags = [];
    for (const match of matches) {
      titleTags.push(match[1]);
    }
    if (titleTags.length !== 0 && !title.endsWith("#")) {
      let currentTagWord = titleTags[titleTags.length - 1];
      allTags = allTags.filter((t) => !titleTags.includes(t));
      allTags = allTags.filter((t) => t.startsWith(currentTagWord));
      allTags.push(currentTagWord);
    }
    return allTags.map((t) => `#${t}`).join(" ");
  }

  async function onTitleChange(e) {
    if (e.keyCode === 32) {
      showHint = undefined;
      intags = false;
    }

    if (title.length !== 0 && e.keyCode === 35) {
      await getUserTagss();
      intags = true;
    }

    if (title.length !== 0 && e.keyCode === 58) {
      showHint = ":tomorrow :yesterday :2022-01-21. (default is today)";
    }

    if (title.length !== 0 && e.keyCode === 64) {
      showHint = "@2.5 (effort of todo in hours, default is 1)";
    }

    if (intags) {
      const s = String.fromCharCode(e.keyCode);
      let text = title;
      if (e.keyCode != 35) text += s;
      showHint = getTagsHint(text, [...usertags]);
      if (!showHint) {
        showHint = "new tag";
      }
    }

    if (!showHint) {
      showHint = "eg. Complete Coding #tag :tomorrow @1.5";
    }

    if (title.length !== 0 && e.keyCode === 13) {
      e.preventDefault();
      const due = $displayoption.todosfilter.dates.from;
      try {
        let todo = {
          title,
          effort: 1.0,
          due: due,
          tags: [],
          done: false,
        };
        todo = adjustForFields(todo);
        if (
          todo.title.length === 0 ||
          todo.title === "" ||
          todo.title === " "
        ) {
          return;
        }
        await addTodoAPI($auth.authtoken, todo);
        dispatch("change");
      } catch (error) {
        console.error(error);
        dispatch("error", error);
      } finally {
        title = "";
        showHint = undefined;
        intags = false;
        dispatch("help", false);
      }
    }
  }

  async function onKeyDown(e) {
    if (title.length === 0 || e.keyCode === 27) {
      showHint = undefined;
    }

    if (title.length !== 0 && e.keyCode === 8) {
      if (title.endsWith(" ")) {
        intags = false;
        showHint = undefined;
        return;
      }

      if (intags) {
        showHint = getTagsHint(title, usertags);
        if (!showHint) {
          showHint = "new tag";
        }
      }
    }
  }
</script>

<div class="todoinput">
  <input
    placeholder="enter a todo"
    type="text"
    bind:value={title}
    on:keypress={onTitleChange}
    on:keyup={onKeyDown}
    autofocus
  />
  {#if showHint}
    <p class="hint">{showHint}</p>
  {/if}
</div>

<style>
  .todoinput {
    margin-top: 1.2rem;
    align-items: center;
    gap: 5px;
  }

  .todoinput input {
    width: 100%;
    height: 2.5rem;
    line-height: 2.5rem;
    font-family: var(--mono);
    font-size: 1rem;
    border: 2px solid var(--plum);
  }

  .hint {
    color: var(--aqua);
    font-size: 90%;
  }

  @media (min-width: 850px) {
    .todoinput input {
      height: 2.5rem;
      line-height: 3rem;
      font-size: 1.3rem;
    }
  }
</style>
