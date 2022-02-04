import dayjs from "dayjs";
import { writable } from "svelte/store";

const capitalize = (s) => {
  if (typeof s !== "string") return "";
  return s.charAt(0).toUpperCase() + s.slice(1);
};

const allTodoFilters = [
  "pending",
  "today",
  "tomorrow",
  "yesterday",
  "daterange",
];

const defaultTodosFilter = {
  filter: "pending",
  dates: getDates("pending"),
  heading: headingForFilter("pending"),
};

const defaultReportRequest = {
  from: new Date(),
  to: new Date(),
  groupby: "day", // "day", // day | week | month | tags
  tags: [],
  timezoneoffset: dayjs().utcOffset() * 60,
  heading: headingForReport({ from: new Date(), to: new Date() }),
};

const internal = writable({
  option: "todos", // todos || report
  todosfilter: defaultTodosFilter,
  reportrequest: defaultReportRequest,
});

const displayoption = {
  subscribe: internal.subscribe,

  filters: () => allTodoFilters,

  setTodosFilter: (filter, daterange = undefined) => {
    const todosfilter = {
      filter: filter,
      dates: getDates(filter, daterange),
      heading: headingForFilter(filter, daterange),
    };

    internal.set({
      option: "todos",
      todosfilter: todosfilter,
      reportrequest: defaultReportRequest,
    });
  },

  setReportRequest: (from, to, groupby, tags) => {
    const reportRequest = {
      from,
      to,
      groupby,
      tags,
      timezoneoffset: defaultReportRequest.timezoneoffset,
      heading: headingForReport({ from, to }),
    };
    internal.set({
      option: "report",
      todosfilter: defaultTodosFilter,
      reportrequest: reportRequest,
    });
  },
};

function getDates(filter, daterange = undefined) {
  let dates = {
    from: new Date(),
    to: new Date(),
  };
  if (filter === "pending") return dates;
  if (filter === "today") return dates;
  if (filter === "tomorrow") {
    const dat = dayjs().add(1, "d").toDate();
    return { from: dat, to: dat };
  }
  if (filter === "yesterday") {
    const dat = dayjs().subtract(1, "d").toDate();
    return { from: dat, to: dat };
  }

  if (daterange) {
    return daterange;
  }

  return dates;
}

function headingForFilter(
  filter,
  daterange = { from: new Date(), to: new Date() }
) {
  if (filter == "daterange") {
    const from = dayjs(daterange.from).format("DD MMM YYYY");
    const to = dayjs(daterange.to).format("DD MMM YYYY");
    if (from === to) {
      return from;
    }
    return `${from} - ${to}`;
  }

  return capitalize(filter);
}

function headingForReport(dates) {
  let heading = "Report";

  const from = dayjs(dates.from).format("DD MMM YYYY");
  const to = dayjs(dates.to).format("DD MMM YYYY");
  if (from === to) {
    return `${heading} for ${from}`;
  }
  return `${heading} for ${from} to ${to}`;
}

/*const internal = writable({
  option: "todos", //or todos | report
  todofilter: "pending",
  reportrequest: undefined,
  heading: headingForTodoFilter("pending"),
});

const displayoption = {
  subscribe: internal.subscribe,

  filters: () => {
    return allFilters();
  },

  set: (option, data) => {
    if (option === "todos") {
      internal.set({
        option: option,
        todofilter: data,
        reportrequest: undefined,
        heading: headingForTodoFilter(data),
      });
      return;
    }

    if (option === "report") {
      internal.set({
        option: option,
        todofilter: undefined,
        reportrequest: data,
        heading: headingForReport(data),
      });
      return;
    }

    throw {
      location: "store::displayoption::set",
      error: `${option} option not supported`,
    };
  },

  getDatesFromFilter: (filter) => {
    return getDates(filter);
  },
};

function allFilters() {
  return ["pending", "today", "yesterday", "tomorrow"];
}

function headingForTodoFilter(filter) {
  if (filter.startsWith("from")) {
    var regex = new RegExp("^from=(.*)&to=(.*)$");
    var arrs = regex.exec(filter);
    if (arrs.length !== 3) {
      throw `helpers.fileters.js::headingForTodoFilter => Filter ${filter} not in from and to format`;
    }

    const from = dayjs(new Date(arrs[1])).format("DD MMM YYYY");
    const to = dayjs(new Date(arrs[2])).format("DD MMM YYYY");
    if (from === to) {
      return from;
    }
    return `${from} - ${to}`;
  }

  if (allFilters().includes(filter)) {
    return capitalize(filter);
  }

  return "Unknown";
}

function headingForReport(reportRequest) {
  let heading = "Report";

  const from = dayjs(reportRequest.from).format("DD MMM YYYY");
  const to = dayjs(reportRequest.to).format("DD MMM YYYY");
  if (from === to) {
    return `${heading} for ${from}`;
  }
  return `${heading} for ${from} to ${to}`;
}

function getDates(filter) {
  let dates = {
    from: new Date(),
    to: new Date(),
  };
  if (filter === "pending") return dates;
  if (filter === "today") return dates;
  if (filter === "tomorrow") {
    const dat = dayjs().add(1, "d").toDate();
    return { from: dat, to: dat };
  }
  if (filter === "yesterday") {
    const dat = dayjs().subtract(1, "d").toDate();
    return { from: dat, to: dat };
  }

  if (filter.startsWith("from")) {
    var regex = new RegExp("^from=(.*)&to=(.*)$");
    var arrs = regex.exec(filter);
    if (arrs.length === 3) {
      return { from: new Date(arrs[1]), to: new Date(arrs[1]) };
    }
  }

  return dates;
}*/

export default displayoption;
