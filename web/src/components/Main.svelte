<script>
  import auth from "../store/auth.js";
  import Login from "./Login.svelte";
  import TodoArea from "./TodoArea.svelte";
</script>

<div class="main">
  {#if $auth.state === "loggedout"}
    <h1>Signin please</h1>
  {:else if $auth.state === "signup"}
    <Login signup={true} />
  {:else if $auth.state === "login"}
    <Login />
  {:else if $auth.state === "loggedin"}
    <TodoArea />
  {:else}
    <h1>Unknown state</h1>
  {/if}
</div>

<style>
  .main {
    width: 100%;
    margin: 0 auto;
  }

  @media (min-width: 850px) {
    .main {
      width: 70%;
      margin: 0 auto;
    }
  }
</style>
