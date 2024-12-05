const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function getWelcomeMsg() {
  return await invoke("welcome");
}

async function getEmail(last) {
  return await invoke("get_email", {last: last})
}

window.addEventListener("DOMContentLoaded", async () => {
  let under = document.querySelector("#under-greet-msg");
  let innerNode = document.createElement("div")
  let welcome = await getWelcomeMsg();
  innerNode.innerHTML = "<p>" + welcome + "</p>";
  under.appendChild(innerNode);
});


window.addEventListener("DOMContentLoaded", async () => {
  let inboxButton = document.getElementById("inbox");
  inboxButton.addEventListener("click", async (e) => {
    e.preventDefault();
    let emailList = await getEmail(0);
    let parentNode = document.getElementById("thumbnail")
    for (let i = 0; i < emailList.length; i++) {
      const content = emailList[i];
      let email = document.createElement("div");
      email.setAttribute("class", "flex-button");
      email.textContent = content;
      parentNode.appendChild(email);
    }
  })
})