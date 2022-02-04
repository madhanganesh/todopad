<script>
  import auth from "../store/auth.js";

  function onSignup() {
    auth.setSignup();
  }
</script>

<h3>Todopad</h3>

<p>A simple app to track your todo items in an easy and efficient way!</p>

<p>
  The app lets you to plan your todo items and track them throughout the day. It
  is fast, simple and intutive.
</p>

<h3>Key Features</h3>
<ul>
  <li>fast and intutive UI to add and track your todos</li>
  <li>tag todos and see how your efforts are spent</li>
  <li>see efforts grouped for each day, week and month</li>
  <li>save and open any links from the todo list view quickly</li>
  <li>fast experience to move pending todo to next day</li>
</ul>

<blockquote>What gets measured is what gets improved!</blockquote>

<p class="signup">
  Please
  <a href="/" on:click|preventDefault={() => auth.setSignup()} class="button"
    >Signup</a
  >
  and write a todo!
</p>

<p>Source @ <a href="https://github.com/madhanganesh/todopad">github</a></p>
<p>
  Contact <a href="mailto:madhanganesh@gmail.com">madhanganesh@gmail.com</a>
</p>

<h3>Application</h3>
<img src="app-screen-shots/todos.png" alt="todo app" />

<h3>Reports by day | week | month</h3>
<img src="app-screen-shots/report-day.png" alt="todo app" />

<h3>Reports by tags</h3>
<img src="app-screen-shots/report-tags.png" alt="todo app" />

<h3>Filters</h3>
<img src="app-screen-shots/filter.png" alt="todo app" />

<h4>Quickly open links or move todo to next day</h4>
<img class="small" src="app-screen-shots/todo-links.png" alt="todo app" />

Please
<a href="/" on:click|preventDefault={() => auth.setSignup()} class="button"
  >Signup</a
> and write a todo!

<style>
  img {
    display: block;
    margin-top: 2rem;
    margin-bottom: 2rem;
    border: 1px solid var(--hotmag);
    border-radius: 5px;
    padding: 0.1rem;
    width: 100%;
    height: 500px;
  }

  .small {
    height: 75px;
  }

  a {
    color: var(--magenta);
  }

  blockquote {
    background-color: var(--dkblue);
    padding: 1rem;
    border-radius: 5px;
    text-align: center;
    margin-top: 2rem;
    margin-bottom: 2rem;
  }
</style>
