<script>
  import auth from "../store/auth.js";
</script>

<header>
  <h1>
    <span><span class="fas fa-tasks" aria-hidden="true" /> todo-pad</span>
  </h1>

  <nav>
    <ul>
      {#if $auth.state === "loggedin"}
        <li>{$auth.username}</li>
        <li><a href="#">About</a></li>
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

<!--header>
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
</header-->

<!--style>
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
</!--header-->
<style>
  header {
    width: 70%;
    font-size: 70%;
    margin: 0 auto;
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
</style>
