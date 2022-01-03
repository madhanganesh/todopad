<script>
  import auth from "../store/auth.js";
  import Login from "./Login.svelte";
</script>

<div>
  {#if $auth.state === "loggedout"}
    <h1>Signin please</h1>
  {:else if $auth.state === "signup"}
    <Login signup={true} />
  {:else if $auth.state === "login"}
    <Login />
  {:else if $auth.state === "loggedin"}
    Show tasks
  {:else}
    <h1>Unknown state</h1>
  {/if}
</div>
