const { invoke } = window.__TAURI__.tauri;

async function generateNewTable(width, height) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let table = document.querySelector("#table");
  table.innerHTML = await invoke("generate_new_table", { height: height.value, width: width.value });
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#table-generation-form").addEventListener("submit", (e) => {
    let width = document.querySelector("#width-input");
    let height = document.querySelector("#height-input");
    e.preventDefault();
    (width.value && height.value) ? generateNewTable(width, height) : null
  });
});
