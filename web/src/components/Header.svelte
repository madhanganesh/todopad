<script>
  import auth from "../store/auth.js";
</script>

<header>
  <nav>
    <ul>
      <li>
        <h1>
          <a href="/"
            ><span class="fas fa-tasks" aria-hidden="true" />
            <span>Todopad</span></a
          >
        </h1>
      </li>
      {#if $auth.state === "loggedin"}
        <li><span class="username">{$auth.username}</span></li>
        <li>
          <a
            href="/"
            on:click|preventDefault={() => auth.setLoggedOff()}
            class="button">Logout</a
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
  nav {
    font-family: var(--mono);
    font-size: 80%;
    padding: 1rem;
  }

  nav h1 a {
    font-family: var(--sans);
  }

  nav ul li:first-child {
    flex-basis: 100%;
    text-align: center;
  }

  nav [class*="fa-"] {
    font-size: 150%;
    color: var(--aqua);
  }

  nav h1 [class*="fa-"] {
    font-size: 90%;
    color: var(--aqua);
  }

  nav ul {
    margin: 0;
    padding: 0;
    list-style-type: none;
    display: flex;
    flex-flow: row wrap;
    justify-content: center;
    align-items: center;
    gap: 2rem;
  }

  nav a {
    text-decoration: none;
    color: var(--white);
    display: block;
  }

  nav a:hover,
  nav [class*="fa-"]:hover {
    color: var(--magenta);
  }

  .button {
    background-color: var(--magenta);
    padding: 0.5rem;
    border-radius: 5px;
  }

  .button:hover {
    color: var(--white);
    background-color: var(--hotmag);
  }

  @media (min-width: 850px) {
    nav ul li:first-child {
      flex-basis: auto;
      text-align: left;
      margin-right: auto;
    }
  }
</style>
