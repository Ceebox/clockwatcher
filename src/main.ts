import { Window } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

let timer: HTMLInputElement | null;
let mEndTime: number | null;

document
  ?.getElementById("titlebar-minimize")
  ?.addEventListener("click", () => Window.getCurrent().minimize());
document
  ?.getElementById("titlebar-close")
  ?.addEventListener("click", () => Window.getCurrent().close());

window.addEventListener("DOMContentLoaded", async () => {
  timer = document.querySelector("#timer");

  await invoke("run_startup").then((response: any) => {
    // If our response is a number, then we have the value from the server
    const outputNum = parseInt(response);

    // Fade in our new value
    if (timer !== null) {
      timer.setAttribute("class", "text-fade");

      setTimeout(() => {
        timer!.setAttribute("class", "text-show");
      }, 1000);
    }

    // Check if we got a number correctly
    if (isNaN(outputNum)) {
      // We have an error code instead
      if (timer !== null) {
        timer.innerHTML = response;
      }

      // Short circuit
      return;
    }

    mEndTime = outputNum;

    setInterval(updateTimer, 1000);
  });
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
