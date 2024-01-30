import { invoke } from "@tauri-apps/api/tauri";
declare var $: any; // for jQuery
declare var Chessboard: any; // for chessboard.js
let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

function pieceTheme (piece: string) {
  return './assets/chessPieces/' + piece + '.svg'
}

window.addEventListener("DOMContentLoaded", () => {
  // console.log(Chessboard);
  // console.log($);
  var config = {
    draggable: true,
    moveSpeed: 'slow',
    snapbackSpeed: 500,
    snapSpeed: 100,
    position: 'start',
    pieceTheme: pieceTheme,
  }
  var board = Chessboard("board", config);
  $("#startBtn").on("click", board.start);
  $("#clearBtn").on("click", board.clear);
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
