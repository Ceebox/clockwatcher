import { listen } from "@tauri-apps/api/event";
import { Window } from "@tauri-apps/api/window";

listen<string>("change-page", (event) => {
  window.location.href = `./${event.payload}`;
});

document
  ?.getElementById("titlebar-minimize")
  ?.addEventListener("click", () => Window.getCurrent().minimize());
document
  ?.getElementById("titlebar-close")
  ?.addEventListener("click", () => Window.getCurrent().close());
document
  ?.getElementById("titlebar-back")
  ?.addEventListener("click", () => (window.location.href = "index.html"));
