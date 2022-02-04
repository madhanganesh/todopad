import dayjs from "dayjs";

let baseURL = "https://todopad.in/api";

if (process.env.NODE_ENV !== "production") {
  baseURL = "http://localhost:8080";
}

export async function signupAPI(name, email, password) {
  const payload = {
    method: "POST",
    body: JSON.stringify({ name, email, password }),
    headers: { "Content-Type": "application/json" },
  };

  const res = await fetch(`${baseURL}/signup`, payload);
  if (!res.ok) {
    const text = await res.text();
    if (res.status === 409) {
      throw "Email already exists. Please login if you have registered already.";
    }
    throw text;
  }

  const result = await res.json();
  return result;
}

export async function loginAPI(email, password) {
  const payload = {
    method: "POST",
    body: JSON.stringify({ email, password }),
    headers: { "Content-Type": "application/json" },
  };

  const res = await fetch(`${baseURL}/login`, payload);
  if (!res.ok) {
    const text = await res.text();
    if (res.status === 404) {
      throw "Email not found. Please login with valid credentials.";
    } else if (res.status === 403) {
      throw "Invalid Credentials. Please login with valid credentials.";
    }
    throw text;
  }

  const result = await res.json();
  return result;
}

function checkErrorForAuth(error) {
  try {
    if (typeof error === "string") {
      if (error.trim() === "404 page not found") {
        error = "Unable to perform this action now. Please try after sometime.";
      } else {
        const errObj = JSON.parse(error);
        if (errObj && errObj.error) {
          error = errObj.error;
          if (error.includes("invalid auth token")) {
            error = "Auth Token Expired. Please logout and login again.";
          }
        }
      }
    }
    return error;
  } catch (e) {
    console.error(e);
    return error;
  }
}

export async function getTodosAPI(token, filter) {
  try {
    if (filter.filter === "pending") {
      const url = `${baseURL}/todo?pending=true`;
      return await getTodosForFilter(token, url);
    }

    const from = dayjs(filter.dates.from).startOf("d").toISOString();
    const to = dayjs(filter.dates.to).endOf("d").toISOString();
    const url = `${baseURL}/todo?from=${from}&to=${to}`;
    return await getTodosForFilter(token, url);

    /*if (filter.startsWith("from")) {
      const url = `${baseURL}/todo?${filter}`;
      return await getTodosForFilter(token, url);
    }

    const range = getFormattedDates(filter);
    const url = `${baseURL}/todo?from=${range.from}&to=${range.to}`;
    return await getTodosForFilter(token, url);*/
  } catch (error) {
    throw checkErrorForAuth(error);
  }
}

async function getTodosForFilter(token, url) {
  const payload = {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
      Origin: "http://127.0.0.1:5000",
    },
  };

  const res = await fetch(url, payload);
  if (!res.ok) {
    const text = await res.text();
    throw text;
  }

  let result = await res.json();
  result = result.map((r) => {
    return { ...r, due: new Date(r.due) };
  });
  return result;
}

export async function addTodoAPI(token, todo) {
  const payload = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
      Origin: "http://127.0.0.1:5000",
    },
    body: JSON.stringify(todo),
  };

  try {
    const url = `${baseURL}/todo`;
    const res = await fetch(url, payload);
    if (!res.ok) {
      const text = await res.text();
      throw text;
    }

    const result = await res.json();
    return result;
  } catch (error) {
    throw checkErrorForAuth(error);
  }
}

export async function updateTodoAPI(token, todo) {
  const payload = {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
      Origin: "http://127.0.0.1:5000",
    },
    body: JSON.stringify(todo),
  };

  try {
    const url = `${baseURL}/todo/${todo.id}`;
    const res = await fetch(url, payload);
    if (!res.ok) {
      const text = await res.text();
      throw text;
    }
  } catch (error) {
    throw checkErrorForAuth(error);
  }
}

export async function deleteTodoAPI(token, todoid) {
  const payload = {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
      Origin: "http://127.0.0.1:5000",
    },
  };

  try {
    const url = `${baseURL}/todo/${todoid}`;
    const res = await fetch(url, payload);
    if (!res.ok) {
      const text = await res.text();
      throw text;
    }
  } catch (error) {
    throw checkErrorForAuth(error);
  }
}

export async function getUserTags(token) {
  const url = `${baseURL}/usertags`;
  const payload = {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
      Origin: "http://127.0.0.1:5000",
    },
  };

  const res = await fetch(url, payload);
  if (!res.ok) {
    const text = await res.text();
    throw text;
  }

  const result = await res.json();
  return result;
}

export async function getAdhocReportAPI(token, reportRequest) {
  const payload = {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
      Origin: "http://127.0.0.1:5000",
    },
    body: JSON.stringify(reportRequest),
  };

  try {
    const url = `${baseURL}/report`;
    const res = await fetch(url, payload);
    if (!res.ok) {
      const text = await res.text();
      throw text;
    }
    const result = await res.json();
    return result;
  } catch (error) {
    throw checkErrorForAuth(error);
  }
}

function getFormattedDates(filter) {
  let dates = getDateRangeForFilter(filter);
  return {
    from: dates.from.toISOString(),
    to: dates.to.toISOString(),
  };
}

function getDateRangeForFilter(filter) {
  let date = getDateForFilter(filter);
  return {
    from: date.startOf("d").toDate(),
    to: date.endOf("d").toDate(),
  };
}

function getDateForFilter(filter) {
  if (filter === "today") return dayjs();
  if (filter === "yesterday") return dayjs().subtract(1, "d");
  if (filter === "tomorrow") return dayjs().add(1, "d");

  throw `Unsuported date filter ${filter}`;
}
