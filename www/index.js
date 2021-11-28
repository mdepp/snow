import { AppState } from "snow";

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const rect = canvas.getBoundingClientRect();
canvas.width = rect.width;
canvas.height = rect.height;

const state = AppState.new();
let prev_time = null;

/** @type {FrameRequestCallback} */
function onFrame(current_time) {
  let duration;
  if (prev_time === null) {
    duration = 0;
  } else {
    duration = current_time - prev_time;
  }
  prev_time = current_time;

  state.tick(duration / 1000.0);
  state.draw(ctx, canvas.width, canvas.height);
  requestAnimationFrame(onFrame);
}

requestAnimationFrame(onFrame);
