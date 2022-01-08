let baseURL = "";

if (process.env.NODE_ENV !== "production") {
  baseURL = "http://127.0.0.1:8080";
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
    console.log(text);
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
    console.log(text);
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

export async function getPendingTodosAPI(token) {
  const payload = {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
      Origin: "http://127.0.0.1:5000",
    },
  };

  const res = await fetch(`${baseURL}/todo?pending=true`, payload);
  if (!res.ok) {
    const text = await res.text();
    console.log(text);
    throw text;
  }

  const result = await res.json();
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

  const res = await fetch(`${baseURL}/todo`, payload);
  if (!res.ok) {
    const text = await res.text();
    console.log(text);
    throw text;
  }

  const result = await res.json();
  return result;
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

  const url = `${baseURL}/todo/${todo.id}`;
  const res = await fetch(url, payload);
  if (!res.ok) {
    const text = await res.text();
    console.log(text);
    throw text;
  }

  //const result = await res.json();
  //return result;
}
