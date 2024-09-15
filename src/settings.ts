import { invoke } from "@tauri-apps/api/core";

let duration: HTMLInputElement | null;

window.addEventListener("DOMContentLoaded", async () => {
  duration = document.querySelector("#duration");

  await invoke("get_duration").then((response: any) => {
    const outputNum = parseInt(response);
    if (duration !== null) {
      let hours = Math.floor(outputNum / 3600000).toLocaleString(
        navigator.language,
        {
          minimumIntegerDigits: 2,
          useGrouping: false,
        },
      );
      let minutes = ((outputNum % 3600000) / 60000).toLocaleString(
        navigator.language,
        {
          minimumIntegerDigits: 2,
          useGrouping: false,
        },
      );

      duration.value = hours + ":" + minutes;

      duration.addEventListener("change", durationChanged);
    }
  });
});

function durationChanged() {
  if (duration !== null) {
    let dateElements = duration.value.split(":");
    let hours = parseInt(dateElements[0]);
    let minutes = parseInt(dateElements[1]);
    let seconds = hours * 60 * 60 + minutes * 60;
    let milliseconds = seconds * 1000;

    invoke("write_duration", { time: milliseconds });
  }
}
