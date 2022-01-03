<script>
  import auth from "./store/auth.js";
  import Header from "./components/Header.svelte";
  import Main from "./components/Main.svelte";

  auth.appStarted();
</script>

<div>
  <Header />
  <Main />
</div>

<style>
</style>
