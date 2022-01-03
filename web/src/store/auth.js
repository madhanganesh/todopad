import { writable } from "svelte/store";

const noUserState = {
  state: "loggedout",
  username: null,
  userid: null,
  authtoken: null,
};

const internal = writable(noUserState);

const auth = {
  subscribe: internal.subscribe,
  appStarted: () => {
    const data = localStorage.getItem("todo-auth-state");
    if (data) {
      let jsonData = JSON.parse(data);
      jsonData = { ...jsonData, state: "loggedin" };
      internal.set(jsonData);
    }
  },
  setLoggedOff: () => {
    internal.set(noUserState);
    localStorage.removeItem("todo-auth-state");
  },
  setSignup: () => {
    const temp = { ...noUserState, state: "signup" };
    internal.set(temp);
  },
  setLogin: () => {
    const temp = { ...noUserState, state: "login" };
    internal.set(temp);
  },
  setLoggedIn: (username, userid, authtoken) => {
    internal.set({
      state: "loggedin",
      username: username,
      userid: userid,
      authtoken: authtoken,
    });
    localStorage.setItem(
      "todo-auth-state",
      JSON.stringify({ username, userid, authtoken })
    );
  },
};

export default auth;
