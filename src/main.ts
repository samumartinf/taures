import { invoke } from "@tauri-apps/api/tauri";
declare var $: any; // for jQuery
declare var Chessboard: any; // for chessboard.js
import blackBishop from './assets/chessPieces/bB.svg';
import blackPawn from './assets/chessPieces/bP.svg';
import blackQueen from './assets/chessPieces/bQ.svg';
import blackRook from './assets/chessPieces/bR.svg';
import blackKing from './assets/chessPieces/bK.svg';
import blackKnight from './assets/chessPieces/bN.svg';
import whiteBishop from './assets/chessPieces/wB.svg';
import whitePawn from './assets/chessPieces/wP.svg';
import whiteQueen from './assets/chessPieces/wQ.svg';
import whiteRook from './assets/chessPieces/wR.svg';
import whiteKnight from './assets/chessPieces/wN.svg';
import whiteKing from './assets/chessPieces/wK.svg';

var config = {
  draggable: true,
  moveSpeed: "fast",
  snapbackSpeed: 100,
  snapSpeed: 100,
  onSnapEnd: onSnapEnd,
  position: "start",
  onDrop: onDrop,
  onDragStart: onDragStart,
  pieceTheme: pieceTheme,
};

var board = Chessboard("board", config);

// update the board position after the piece snap
// for castling, en passant, pawn promotion
async function onSnapEnd() {
  board.position(await invoke("get_fen_simple"));
}

// all paths start from the root of the project (i.e. folder with index.html
function pieceTheme(piece: string) {
  switch (piece) {
    case "wB":
      return whiteBishop;
    case "bB":
      return blackBishop;
    case "wP":
      return whitePawn;
    case "bP":
      return blackPawn;
    case "wQ":
      return whiteQueen;
    case "bQ":
      return blackQueen;
    case "wR":
      return whiteRook;
    case "bR":
      return blackRook;
    case "wN":
      return whiteKnight;
    case "bN":
      return blackKnight;
    case "bK":
      return blackKing;
    case "wK":
      return whiteKing;
    default:
      return null;
  }
}

async function onDragStart(source: string, piece) {
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


//TODO: Figure out which where the text of the input is stored
async function setFen() {
  var input = document.getElementById("FenInput")
  console.log(input);
  console.log(input?.innerHTML);
  if (input) {
    var fen: string = input.innerText;
    var success = await invoke("set_fen", {fen: fen})
    if (success) {
      board.position(fen);
    }
  }
}

async function onDrop(
  source: string,
  target: string,
  piece: string,
) {

  var promotion_piece: string = "";
  if (piece === "p" && target[1] === "1") {
    // promotion for black
    promotion_piece = "q";
  } else if (piece === "P" && target[1] === "8") {
    promotion_piece = "Q";
  }

  // Check if move is legal
  var move = {
    source: source,
    target: target,
    promotion: promotion_piece,
  };

  var isLegal: boolean = await invoke("play_move", move);

  console.log("Move: " + move + "has legality=" + isLegal);

  // if (!move) {
  //   return "snapback";
  // }
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
  $("#setFenBtn").on("click", setFen);
});
