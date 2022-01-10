<script>
  import auth from "../store/auth.js";
  import { loginAPI, signupAPI } from "../helpers/api.js";
  import TextInput from "../UI/TextInput.svelte";
  import { isEmpty, isValidEmail } from "../helpers/validation.js";

  export let signup = false;

  let title = signup ? "Signup" : "Login";
  let name = "";
  let email = "";
  let password = "";
  let showError = null;

  $: isFormValid = () => {
    let valid = !isEmpty(password) && !isValidEmail(email);
    if (signup) {
      valid = valid && !isEmpty(name);
    }
    return valid;
  };

  function onCancel() {
    auth.setLoggedOff();
  }

  async function onSubmit() {
    showError = null;
    if (!isFormValid()) {
      showError = "invalid form";
      return;
    }

    try {
      let result;
      if (signup) {
        result = await signupAPI(name, email, password);
      } else {
        result = await loginAPI(email, password);
      }
      auth.setLoggedIn(result.name, result.userid, result.token);
    } catch (error) {
      showError = error;
    }
  }
</script>

<form>
  <h2>{title}</h2>

  {#if signup}
    <TextInput
      id="name"
      label="Name"
      type="text"
      value={name}
      valid={!isEmpty(name)}
      validityMessage="Please enter your name"
      on:input={(e) => (name = e.detail.target.value)}
      autofocus="true"
    />
  {/if}

  <TextInput
    id="email"
    label="Email"
    type="email"
    value={email}
    valid={!isValidEmail(email)}
    validityMessage="Please enter valid email"
    on:input={(e) => (email = e.detail.target.value)}
    autofocus={!signup}
  />

  <TextInput
    id="password"
    label="Password"
    type="password"
    value={password}
    valid={!isEmpty(password)}
    validityMessage="Please enter your password"
    on:input={(e) => (password = e.detail.target.value)}
  />

  <div class="controls">
    <button
      disabled={!isFormValid()}
      on:click|preventDefault|stopPropagation={onSubmit}>{title}</button
    >
    <button on:click={onCancel}>Cancel</button>
    {#if showError}
      <p class="error">
        {showError}
      </p>
    {/if}
  </div>
</form>

<style>
  form {
    width: 90%;
    border: 1px solid var(--magenta);
    border-radius: 5px;
    padding: 1.2rem;
    margin: 20% auto;
  }

  form .error {
    color: var(--red);
  }

  h2 {
    font-size: 98%;
    color: var(--magenta);
  }

  .controls {
    margin-top: 1.2rem;
  }

  button {
    font: inherit;
    border: 1px solid var(--magenta);
    background: var(--magenta);
    padding: 0.2rem 0.5rem;
    color: var(--white);
    border-radius: 5px;
    box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.26);
    cursor: pointer;
    margin-right: 1.2rem;
  }

  button:hover {
    color: var(--white);
    background-color: var(--hotmag);
    cursor: pointer;
  }

  button:disabled {
    color: grey;
    cursor: default;
    background: var(--magenta);
  }

  @media (min-width: 850px) {
    form {
      width: 70%;
    }
  }
</style>
