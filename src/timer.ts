import { invoke } from "@tauri-apps/api/core";

let timer: HTMLInputElement | null;
let mEndTime: number | null;

window.addEventListener("DOMContentLoaded", async () => {
  timer = document.querySelector("#timer");

  await invoke("run_timer_startup").then((response: any) => {
    // If our response is a number, then we have the value from the server
    const outputNum = parseInt(response);

    // Fade in our new value
    if (timer !== null) {
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

      return;
    }

    mEndTime = outputNum;

    setInterval(updateTimer, 1000);
  });
});

function getTimeRemaining(endTime: number): string {
  let t = Math.abs(endTime - Date.now());
  let minutes = Math.max(Math.ceil((t / 1000 / 60) % 60), 0);
  let hours = Math.max(Math.floor((t / (1000 * 60 * 60)) % 24), 0);
  return (
    hours.toLocaleString(navigator.language, {
      minimumIntegerDigits: 2,
      useGrouping: false,
    }) +
    ":" +
    minutes.toLocaleString(navigator.language, {
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
