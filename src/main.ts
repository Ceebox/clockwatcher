import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";

function changePage(event: String) {
  for (const element of document.getElementsByClassName("text-show")) {
    element.className = "text-fade";
  }
  setTimeout(function () {
    window.location.href = `./${event}`;
  }, 1000);
}

// Make sure we change the page when the user wants us to
listen<string>("change-page", (event) => {
  changePage(event.payload);
});

// Add the theme CSS to the document
await invoke("get_theme").then((theme: any) => {
  let head = document.getElementsByTagName("head")[0];
  let link = document.createElement("link");
  link.rel = "stylesheet";
  link.href = "/src/theme-" + theme + ".css";
  link.media = "all";
  head.appendChild(link);
});

// Assign nav button function
document
  ?.getElementById("titlebar-minimize")
  ?.addEventListener("click", () => Window.getCurrent().minimize());
document
  ?.getElementById("titlebar-close")
  ?.addEventListener("click", () => Window.getCurrent().close());
document
  ?.getElementById("titlebar-back")
  ?.addEventListener("click", () => changePage("index.html"));
