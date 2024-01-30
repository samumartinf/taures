import { invoke } from "@tauri-apps/api/tauri";
declare var $: any; // for jQuery
declare var Chessboard: any; // for chessboard.js
let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

var config = {
  draggable: true,
  moveSpeed: 'slow',
  snapbackSpeed: 500,
  snapSpeed: 100,
  onSnapEnd: onSnapEnd,
  position: 'start',
  onDrop: onDrop,
  pieceTheme: pieceTheme,
};

var board = Chessboard("board", config);

// update the board position after the piece snap
// for castling, en passant, pawn promotion
async function onSnapEnd () {
  board.position(await invoke("get_fen"));
}

// all paths start from the root of the project (i.e. folder with index.html)
function pieceTheme(piece: string) {
  return 'src/assets/chessPieces/' + piece + '.svg';
}

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function restart() {
  console.log('Restarting game...');  
  await invoke("restart_game");
  board.start();
}

async function undoMove() {
  await invoke("undo_move");
  var fen: string = await invoke("get_fen");
  board.position(fen);
}

async function onDrop(source: string, target: string, piece: string, newPos, oldPos, orientation) {
  console.log('Source: ' + source);
  console.log('Target: ' + target);
  console.log('Piece: ' + piece);
  console.log('New position: ' + Chessboard.objToFen(newPos));
  console.log('Old position: ' + Chessboard.objToFen(oldPos));
  console.log('Orientation: ' + orientation);
  console.log('~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~');

  // Check if move is legal
  var move: boolean = await invoke("play_move", {
    source: source,
    target: target,
  });

  console.log('Move: ' + move); 

  if (!move) {
    return 'snapback';
  }
}

window.addEventListener("DOMContentLoaded", () => {

  $("#startBtn").on("click", restart);
  // $("#clearBtn").on("click", board.clear);
  $('#undoBtn').on('click', undoMove);

  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
