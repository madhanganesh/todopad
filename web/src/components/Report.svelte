<script>
  import { onDestroy } from "svelte";

  import auth from "../store/auth.js";
  import displayoption from "../store/displayoption.js";
  import { getAdhocReportAPI } from "../helpers/api.js";

  import Summary from "./Summary.svelte";

  let reportResponse = undefined;
  const unsubscribe = displayoption.subscribe(async (data) => {
    if (data.option !== "report") {
      return;
    }
    try {
      reportResponse = await getAdhocReportAPI(
        $auth.authtoken,
        data.reportrequest
      );
    } catch (error) {
      alert(error);
    }
  });

  onDestroy(() => unsubscribe());

  function capitalize(s) {
    if (typeof s !== "string") return "";
    return s.charAt(0).toUpperCase() + s.slice(1);
  }

  function convert(effortsMap) {
    let efforts = [];
    for (const key in effortsMap) {
      efforts.push({ key: key, value: effortsMap[key] });
    }
    return efforts;
  }

  function summaryCount(effortsMap) {
    let total = 0;
    for (const key in effortsMap) {
      total += effortsMap[key];
    }
    return `${total} hours`;
  }
</script>

<div>
  {#if reportResponse}
    <Summary
      heading={$displayoption.reportrequest.heading}
      countSummary={summaryCount(reportResponse.efforts)}
    />
    <table>
      <tr>
        <th>{capitalize(reportResponse.groupby)}</th>
        <th>Effort (in hours)</th>
      </tr>
      {#each convert(reportResponse.efforts) as effort (effort.key)}
        <tr>
          <td>{effort.key}</td>
          <td>{effort.value}</td>
        </tr>
      {/each}
    </table>
  {/if}
</div>

<style>
  table {
    width: 100%;
    border-collapse: collapse;
  }

  td,
  th {
    border: 1px solid var(--aqua);
    text-align: left;
    padding: 8px;
  }

  th {
    color: var(--aqua);
  }
</style>
