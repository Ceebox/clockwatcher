import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";

// Make sure we change the page when the user wants us to
listen<string>("change-page", (event) => {
  window.location.href = `./${event.payload}`;
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

// Add nav buttons to the page
document
  ?.getElementById("titlebar-minimize")
  ?.addEventListener("click", () => Window.getCurrent().minimize());
document
  ?.getElementById("titlebar-close")
  ?.addEventListener("click", () => Window.getCurrent().close());
document
  ?.getElementById("titlebar-back")
  ?.addEventListener("click", () => (window.location.href = "index.html"));
