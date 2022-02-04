<script>
  import { onMount } from "svelte";
  import dayjs from "dayjs";
  import Tags from "svelte-tags-input";

  import { createEventDispatcher } from "svelte";

  import auth from "../store/auth.js";
  import todos from "../store/todos.js";

  import TextInput from "../UI/TextInput.svelte";
  import Button from "../UI/Button.svelte";
  import Modal from "../UI/Modal.svelte";
  import { isEmpty } from "../helpers/validation.js";

  import { updateTodoAPI, deleteTodoAPI, getUserTags } from "../helpers/api.js";

  export let id = null;

  let todo = null;
  let formIsValid = false;
  let usertags = [];

  onMount(async () => {
    try {
      const usertagsres = await getUserTags($auth.authtoken);
      usertags = usertagsres.tags;
    } catch (e) {
      usertags = [];
      console.error(e);
    }
  });

  function handleTags(event) {
    todo.tags = event.detail.tags;
  }

  if (id) {
    const unsubscribe = todos.subscribe((items) => {
      todo = items.find((i) => i.id === id);
    });

    unsubscribe();
  }

  const dispatch = createEventDispatcher();

  $: formIsValid = !isEmpty(todo.title);

  async function submitForm() {
    try {
      todo.due = new Date(todo.due);
      todo.effort = parseFloat(todo.effort);
      await updateTodoAPI($auth.authtoken, todo);
    } catch (error) {
      dispatch("error", error);
    } finally {
      dispatch("save");
    }

    /*if (id) {
      fetch(
        `https://svelte-proj-9e2ad-default-rtdb.asia-southeast1.firebasedatabase.app/meetups/${id}.json`,
        {
          method: "PATCH",
          body: JSON.stringify(meetup),
          headers: { "Content-Type": "application/json" },
        }
      )
        .then((res) => {
          if (!res.ok) {
            throw new Error("An error occurred");
          }
          meetups.updateMeetup(id, meetup);
        })
        .catch((err) => {
          alert(err);
        });
    } else {
      fetch(
        "https://svelte-proj-9e2ad-default-rtdb.asia-southeast1.firebasedatabase.app/meetups.json",
        {
          method: "POST",
          body: JSON.stringify(meetup),
          headers: { "Content-Type": "application/json" },
        }
      )
        .then((res) => {
          if (!res.ok) {
            throw new Error("An error occurred");
          }
          return res.json();
        })
        .then((data) => {
          meetups.addMeetup({ ...meetup, id: data.name });
        })
        .catch((err) => {
          alert(err);
        });
    }
    dispatch("save");*/
  }

  function cancel() {
    dispatch("cancel");
  }

  async function deleteMeetup() {
    /*fetch(
      `https://svelte-proj-9e2ad-default-rtdb.asia-southeast1.firebasedatabase.app/meetups/${id}.json`,
      {
        method: "DELETE",
      }
    )
      .then((res) => {
        if (!res.ok) {
          throw new Error("An error occurred");
        }
        meetups.deleteMeetup(id);
      })
      .catch((err) => {
        alert(err);
      });*/

    try {
      await deleteTodoAPI($auth.authtoken, id);
    } catch (error) {
      console.error(error);
    } finally {
      dispatch("cancel");
    }
  }
</script>

<Modal title="Edit Todo">
  <div class="form">
    <TextInput
      id="title"
      label="Title"
      type="text"
      value={todo.title}
      valid={!isEmpty(todo.title)}
      validityMessage="Please enter a valid title"
      autofocus={1}
      on:input={(e) => (todo.title = e.detail.target.value)}
    />

    <TextInput
      id="subtitle"
      label="Subtitle"
      type="date"
      value={dayjs(todo.due).format("YYYY-MM-DD")}
      validityMessage="Please enter a valid due date"
      on:input={(e) => (todo.due = e.detail.target.value)}
    />

    <TextInput
      id="effort"
      label="Effort"
      type="number"
      value={todo.effort}
      valid={todo.effort !== 0}
      validityMessage="Please enter a valid effort"
      on:input={(e) => (todo.effort = e.detail.target.value)}
    />

    <label for="tags">Tags</label>
    <Tags
      tags={todo.tags}
      onlyUnique="true"
      autoComplete={usertags}
      on:tags={handleTags}
    />

    <TextInput
      id="notes"
      label="Notes"
      controlType="textarea"
      value={todo.notes}
      on:input={(e) => (todo.notes = e.detail.target.value)}
    />
  </div>

  <div slot="footer">
    <Button type="button" mode="outline" on:click={cancel}>Cancel</Button>
    <Button type="button" on:click={submitForm} disabled={!formIsValid}
      >Save</Button
    >
  </div>
</Modal>

<style>
  .form {
    width: 100%;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    width: 100%;
    font-weight: bold;
  }
</style>
