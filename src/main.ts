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
  board.position(await invoke("get_fen_simple"));
}

// all paths start from the root of the project (i.e. folder with index.html)
function pieceTheme(piece: string) {
  return 'src/assets/chessPieces/' + piece + '.svg';
}

async function restart() {
  await invoke("restart_game");
  var fen = await invoke("get_fen_simple");
  console.log('FEN from engine: ' + fen);
  board.position(fen);
}

async function undoMove() {
  console.log("Undo move...");
  await invoke("undo_move");
  var fen: string = await invoke("get_fen_simple");
  console.log('FEN from engine: ' + fen);
  board.position(fen);
}

function getFen() {
  console.log("Board FEN: " + board.fen());
}

async function onDrop(source: string, target: string, piece: string, newPos, oldPos, orientation) {

  var fen: string = await invoke("get_fen_simple");
  console.log('Source: ' + source);
  console.log('Target: ' + target);
  console.log('Piece: ' + piece);
  console.log('New position: ' + Chessboard.objToFen(newPos));
  console.log('Old position: ' + Chessboard.objToFen(oldPos));
  console.log('FEN from engine: ' + fen);

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
  $('#undoBtn').on('click', undoMove);
  $('#getFenBtn').on('click', getFen);
});
