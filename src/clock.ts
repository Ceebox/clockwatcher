let mClock: HTMLInputElement | null;

window.addEventListener("DOMContentLoaded", async () => {
  mClock = document.querySelector("#timer");

  // Fade in our new value
  if (mClock !== null) {
    setTimeout(() => {
      mClock!.setAttribute("class", "text-show");
    }, 1000);
  }

  setInterval(updateTimer, 1000);
});

function getTime(): string {
  return new Date().toLocaleTimeString(
    Intl.NumberFormat().resolvedOptions().locale,
    {
      hour: "numeric",
      minute: "numeric",
    },
  );
}

function updateTimer(): void {
  if (mClock != null) {
    mClock.innerHTML = getTime();
  }
}
