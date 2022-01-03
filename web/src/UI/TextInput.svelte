<script>
  import { onMount, createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let id;
  export let label;
  export let value;
  export let type;
  export let autofocus = 0;

  export let valid = true;
  export let validityMessage = "";

  let elm;
  onMount(() => {
    if (autofocus) {
      elm.focus();
    }
  });

  let isDirty = false;
  function onInput(e) {
    isDirty = true;
    dispatch("input", e);
  }

  $: showValidityMessage = !valid && validityMessage && isDirty;
</script>

<div class="form-control">
  <label for={id}>{label}</label>
  <input
    bind:this={elm}
    class:invalid={showValidityMessage}
    {type}
    {id}
    {value}
    on:input={onInput}
  />
  {#if showValidityMessage}
    <p class="error-message">{validityMessage}</p>
  {/if}
</div>

<style>
  input {
    font: inherit;
    display: block;
    width: 100%;
    font: inherit;
    border: none;
    border-bottom: 2px solid #ccc;
    background: white;
    padding: 0.15rem 0.25rem;
    transition: border-color 0.1s ease-out;
  }

  input:focus {
    border-color: var(--magenta);
    outline: none;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    width: 100%;
  }

  .form-control {
    padding: 0.5rem 0;
    width: 100%;
    margin: 0.25rem 0;
  }

  .invalid {
    border-color: red;
    background-color: #fde3e3;
  }

  .error-message {
    color: red;
    margin: 0.25rem 0;
  }
</style>
