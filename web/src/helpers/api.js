export async function signupAPI(name, email, password) {
  const payload = {
    method: "POST",
    body: JSON.stringify({ name, email, password }),
    headers: { "Content-Type": "application/json" },
  };

  const res = await fetch("http://127.0.0.1:8080/signup", payload);
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

  const res = await fetch("http://127.0.0.1:8080/login", payload);
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
