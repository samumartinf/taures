import { invoke } from "@tauri-apps/api/tauri";
declare var $: any; // for jQuery
declare var Chessboard: any; // for chessboard.js

var config = {
  draggable: true,
  moveSpeed: "fast",
  snapbackSpeed: 100,
  snapSpeed: 100,
  onSnapEnd: onSnapEnd,
  position: "start",
  onDrop: onDrop,
  onDragStart: onDragStart2,
  pieceTheme: pieceTheme,
};

var board = Chessboard("board", config);

// update the board position after the piece snap
// for castling, en passant, pawn promotion
async function onSnapEnd() {
  board.position(await invoke("get_fen_simple"));
}

// all paths start from the root of the project (i.e. folder with index.html)
function pieceTheme(piece: string) {
  return "src/assets/chessPieces/" + piece + ".svg";
}

async function onDragStart(source, piece, position, orientation) {
  var fen: string = board.fen();
  // Update debugInfoBoard and debugInfoEngine labels
  var debugLabelBoard = document.getElementById("debugLabelBoard");
  if (debugLabelBoard) {
    debugLabelBoard.innerText = "Board FEN: " + fen + ", Piece: " + piece;
  }
  var fenFromEngine: string = await invoke("get_fen_simple");
  var enginePiece = await invoke("get_piece_at_square", { square: source });

  var debugLabelEngine = document.getElementById("debugLabelEngine");
  if (debugLabelEngine) {
    debugLabelEngine.innerText =
      "Engine FEN: " + fenFromEngine + ", Piece: " + enginePiece;
  }
}

async function onDragStart2(source: string, piece, position, orientation) {
  var fen: string = board.fen();
  // Update debugInfoBoard and debugInfoEngine labels
  var debugLabelBoard = document.getElementById("debugLabelBoard");
  if (debugLabelBoard) {
    debugLabelBoard.innerText = "Board FEN: " + fen + ", Piece: " + piece;
  }

  var fenFromEngine: string = await invoke("get_fen");
  var enginePiece = await invoke("get_piece_at_square", { square: source });

  var debugLabelEngine = document.getElementById("debugLabelEngine");
  if (debugLabelEngine) {
    debugLabelEngine.innerText =
      "Engin FEN: " + fenFromEngine + ", Piece: " + enginePiece;
  }

  var possible_moves_from_engine: [string] = await invoke(
    "get_possible_moves",
    { source: source }
  );

  var legal_moves = await invoke("get_legal_moves", { source: source });
  var allowedMovesEl = document.getElementById("allowedMoves");
  if (allowedMovesEl) {
    allowedMovesEl.innerText =
      "Allowed movez: " +
      possible_moves_from_engine +
      ", Legal moves: " +
      legal_moves;
  }
}

async function makeRandomMove() {
  var newFen: string = await invoke("make_random_move");
  if (newFen == "None") {
    return;
  }
  board.position(newFen);
}

async function makeBestMove() {
  var newFen: string = await invoke("play_best_move", { depth: 2 });
  board.position(newFen);
}

async function restart() {
  await invoke("restart_game");
  var fen = await invoke("get_fen_simple");
  board.position(fen);
}

async function undoMove() {
  await invoke("undo_move");
  var fen: string = await invoke("get_fen_simple");
  board.position(fen);
}

function getFen() {
  console.log("Board FEN: " + board.fen());
}

async function onDrop(
  source: string,
  target: string,
  piece: string,
  newPos,
  oldPos,
  orientation
) {
  // Check if move is legal
  var move: boolean = await invoke("play_move", {
    source: source,
    target: target,
  });

  console.log("Move: " + move);

  if (!move) {
    return "snapback";
  }
}

async function showPosition() {
  await invoke("get_position_string");
  return;
}
window.addEventListener("DOMContentLoaded", () => {
  $("#startBtn").on("click", restart);
  $("#undoBtn").on("click", undoMove);
  $("#getFenBtn").on("click", getFen);
  $("#showPositionBtn").on("click", showPosition);
  $("#randomMoveBtn").on("click", makeRandomMove);
  $("#bestMoveBtn").on("click", makeBestMove);
});
