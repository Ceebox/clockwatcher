import { Window } from "@tauri-apps/api/window";

let timer: HTMLInputElement | null;
let mEndTime: number | null = Date.now() + 62 * 60 * 1000;

document
  ?.getElementById("titlebar-minimize")
  ?.addEventListener("click", () => Window.getCurrent().minimize());
document
  ?.getElementById("titlebar-close")
  ?.addEventListener("click", () => Window.getCurrent().close());

window.addEventListener("DOMContentLoaded", () => {
  timer = document.querySelector("#timer");

  setInterval(updateTimer, 1000);
});

function getTimeRemaining(endTime: number): string {
  let t = Math.abs(endTime - Date.now());
  let minutes = Math.floor((t / 1000 / 60) % 60);
  let hours = Math.floor((t / (1000 * 60 * 60)) % 24);
  return (
    hours.toLocaleString("en-UK", {
      minimumIntegerDigits: 2,
      useGrouping: false,
    }) +
    ":" +
    minutes.toLocaleString("en-UK", {
      minimumIntegerDigits: 2,
      useGrouping: false,
    })
  );
}

function updateTimer(): void {
  if (mEndTime != null && timer != null) {
    timer.innerHTML = getTimeRemaining(mEndTime);
  }
}
