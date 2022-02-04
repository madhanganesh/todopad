<script>
  import dayjs from "dayjs";

  import { onMount, createEventDispatcher } from "svelte";
  import Tags from "svelte-tags-input";

  import auth from "../store/auth.js";
  import displayoption from "../store/displayoption.js";
  import { getUserTags, loginAPI } from "../helpers/api.js";

  import Modal from "../UI/Modal.svelte";
  import Button from "../UI/Button.svelte";

  const dispatch = createEventDispatcher();
  let showTags = false;
  let groupByOptions = ["nothing", "day", "week", "month", "tags"];
  let groupBySelected = "nothing";
  let usertags = [];
  let tagsToGroup = [];
  let customFilter = {
    from: new Date(),
    to: new Date(),
  };
  let error = undefined;

  function initializeTodosFilter(todosfilter) {
    if (todosfilter.filter === "daterange") {
      customFilter.from = todosfilter.dates.from;
      customFilter.to = todosfilter.dates.to;
    }
  }

  function initializeReportRequest(reportRequest) {
    customFilter.from = reportRequest.from;
    customFilter.to = reportRequest.to;
    groupBySelected = reportRequest.groupby;
    if (groupBySelected === "tags") {
      showTags = true;
      tagsToGroup = reportRequest.tags;
    }
  }

  onMount(async () => {
    const unsubscribe = displayoption.subscribe((data) => {
      if (data.option == "todos") initializeTodosFilter(data.todosfilter);
      if (data.option == "report") initializeReportRequest(data.reportrequest);
    });
    unsubscribe();

    try {
      const tagsRes = await getUserTags($auth.authtoken);
      usertags = tagsRes.tags;
    } catch (error) {
      console.error(error);
    }
  });

  function setTodosFilter(filter, daterange = undefined) {
    displayoption.setTodosFilter(filter, daterange);
    dispatch("close");
  }

  function setDisplayOption() {
    if (dayjs(customFilter.to).diff(customFilter.from) < 0) {
      error = "To date is later than from date. Please correct.";
      setTimeout(() => (error = undefined), 3000);
      return;
    }

    if (groupBySelected === "nothing") {
      displayoption.setTodosFilter("daterange", customFilter);
    } else {
      displayoption.setReportRequest(
        dayjs(customFilter.from).startOf("d").toDate(),
        dayjs(customFilter.to).endOf("d").toDate(),
        groupBySelected,
        tagsToGroup
      );
    }
    dispatch("close");
  }

  function cancel() {
    dispatch("close");
  }

  function handleTags(event) {
    tagsToGroup.concat(event.detail.tags);
  }

  function handleGroupByChange() {
    showTags = groupBySelected === "tags";
  }

  const capitalize = (s) => {
    if (typeof s !== "string") return "";
    return s.charAt(0).toUpperCase() + s.slice(1);
  };
</script>

<Modal title="Select Filter">
  <div class="form">
    <ul class="filters">
      {#each displayoption.filters() as filter (filter)}
        {#if filter !== "daterange"}
          <li on:click={() => setTodosFilter(filter)}>
            {capitalize(filter)}
          </li>
        {/if}
      {/each}
      <li>
        Custom
        <div on:click class="custom">
          <div class="fields">
            <span class="field">
              <label for="fromdate">From</label>
              <input
                id="fromdate"
                type="date"
                value={dayjs(customFilter.from).format("YYYY-MM-DD")}
                on:input={(e) => {
                  customFilter.from = dayjs(e.target.value)
                    .add(10, "m")
                    .toDate();
                  customFilter.to = customFilter.from;
                }}
              />
            </span>
            <span class="field">
              <label for="todate">To</label>
              <input
                id="todate"
                type="date"
                value={dayjs(customFilter.to).format("YYYY-MM-DD")}
                on:input={(e) => {
                  customFilter.to = dayjs(e.target.value).add(10, "m").toDate();
                }}
              />
            </span>

            <span class="field">
              <label for="dategroup">Group by</label>
              <select
                bind:value={groupBySelected}
                on:change={handleGroupByChange}
              >
                {#each groupByOptions as option (option)}
                  <option value={option}>
                    {option}
                  </option>
                {/each}
              </select>
            </span>
            {#if showTags}
              <span class="field">
                <Tags
                  labelText="Sub-group by"
                  labelShow={false}
                  onlyUnique="true"
                  tags={tagsToGroup}
                  autoComplete={usertags}
                  placeholder="Sub-group by tags (optional)"
                  on:tags={handleTags}
                />
              </span>
            {/if}
          </div>
        </div>
      </li>
    </ul>
    {#if error}
      <p class="error">{error}</p>
    {/if}
  </div>
  <div slot="footer">
    <Button type="button" mode="outline" on:click={cancel}>Cancel</Button>
    <Button type="button" mode="outline" on:click={setDisplayOption}
      >Select Custom Filter</Button
    >
  </div>
</Modal>

<style>
  .filters {
    list-style-type: none;
    padding: 0;
    margin: 0;
  }

  .filters li {
    cursor: pointer;
    padding: 5px;
    margin-bottom: 5px;
  }

  .filters li:hover {
    background-color: var(--aqua);
  }

  .filters li:last-child:hover {
    background-color: white;
  }

  .custom .fields {
    border: 1px solid black;
    margin-top: 5px;
    padding: 5px;
  }

  .custom .field {
    margin-top: 10px;
    display: block;
  }

  .custom .field label {
    width: 100px;
    display: inline-block;
  }

  .custom select,
  .custom input {
    line-height: 1.8rem;
    height: 1.8rem;
    font-size: 1.4rem;
    cursor: pointer;
  }

  .error {
    color: var(--red);
  }
</style>
