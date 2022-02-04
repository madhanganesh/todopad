<script>
  import auth from "../store/auth.js";

  import Home from "./Home.svelte";
  import Login from "./Login.svelte";
  import TodoArea from "./TodoArea.svelte";
  import About from "./About.svelte";
</script>

<div class="main">
  {#if $auth.state === "loggedout"}
    <Home />
  {:else if $auth.state === "signup"}
    <Login signup={true} />
  {:else if $auth.state === "login"}
    <Login />
  {:else if $auth.state === "loggedin"}
    <TodoArea />
  {:else}
    <About />
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
