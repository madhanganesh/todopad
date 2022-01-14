<script>
  let commands = [
    { key: "?", action: "show the pending todos" },
    { key: "?today", action: "show todos of today" },
    { key: "?yesterday", action: "show todos of yesterday" },
    { key: "?tomorrow", action: "show todos of tomorrow" },
    { key: "?this week", action: "show todos of this week" },
  ];
</script>

<p class="summary">Type the command and press enter key for the action:</p>
<ul class="command">
  {#each commands as command (command.key)}
    <li>
      <span class="command-key">{command.key}</span> -
      <span class="command-action">{command.action}</span>
    </li>
  {/each}
</ul>

<style>
  .summary {
    font-size: 1.2rem;
    color: var(--magenta);
    display: flex;
  }

  ul.command {
    margin: 0;
    padding: 0;
  }

  ul.command li {
    list-style-type: none;
    color: var(--aqua);
    margin-bottom: 0.5rem;
    font-size: 1rem;
  }

  ul.command li .command-key {
    width: 6rem;
    display: inline-block;
  }

  ul.command li .command-action {
    display: inline-block;
    margin-left: 1rem;
  }

  @media (min-width: 850px) {
    .todos ul li {
      font-size: 1.3rem;
    }

    ul.command li {
      list-style-type: none;
      color: var(--aqua);
      margin-bottom: 0.5rem;
      font-size: 1.2rem;
    }

    ul.command li .command-key {
      width: 8rem;
      display: inline-block;
    }

    ul.command li .command-action {
      display: inline-block;
      margin-left: 1.2rem;
    }
  }
</style>
