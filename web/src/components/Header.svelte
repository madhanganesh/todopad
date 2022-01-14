<script>
  import auth from "../store/auth.js";
</script>

<header>
  <h1>
    <span><span class="fas fa-tasks" aria-hidden="true" /> todopad</span>
  </h1>

  <nav>
    <ul>
      {#if $auth.state === "loggedin"}
        <li class="username">{$auth.username}</li>
        <li>
          <a href="/" on:click|preventDefault={() => auth.setLoggedOff()}
            >Logout</a
          >
        </li>
      {:else}
        <li>
          <a
            href="/"
            on:click|preventDefault={() => auth.setLogin()}
            class="button">Login</a
          >
        </li>
        <li>
          <a
            href="/"
            on:click|preventDefault={() => auth.setSignup()}
            class="button">Signup</a
          >
        </li>
      {/if}
    </ul>
  </nav>
</header>

<style>
  header {
    width: 100%;
    font-size: 60%;
    padding: 1rem;
    font-family: var(--mono);
    display: flex;
    align-items: center;
  }

  h1 {
    font-family: var(--sans);
  }

  header h1 [class*="fa-"] {
    font-size: 80%;
    color: var(--magenta);
  }

  nav {
    margin-left: auto;
    font-size: 130%;
  }
  nav ul {
    padding: 0;
    margin: 0;
    list-style-type: none;
    display: flex;
    gap: 1rem;
  }

  nav ul li {
    font-family: var(--sans);
  }

  nav a {
    font-family: var(--sans);
    text-decoration: none;
    color: var(--magenta);
    display: block;
  }

  nav a:hover {
    color: var(--hotmag);
  }

  .username {
    visibility: hidden;
  }

  @media (min-width: 850px) {
    header {
      width: 70%;
      font-size: 70%;
      margin: 0 auto;
      font-family: var(--mono);
      display: flex;
      align-items: center;
    }

    .username {
      visibility: visible;
    }
  }
</style>
