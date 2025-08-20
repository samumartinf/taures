use cherris::*;
use std::collections::HashSet;
const PIECE_BIT: u8 = 128u8;
const WHITE_BIT: u8 = 64u8;
const PAWN_BIT: u8 = 8u8;
// const CHECK_PIECE: u8 = 0b00001111;
const KING: u8 = 0u8;
const QUEEN: u8 = 1u8;
const BISHOP: u8 = 2u8;
const KNIGHT: u8 = 4u8;
const ROOK: u8 = 6u8;
// const ROW: u8 = 16u8;
// const COL: u8 = 1u8;

use crate::position_helper;
use cherris::board::Board;
use cherris::engine::Engine;
use cherris::piece::{BasicPiece, Piece, PieceType};
use std::time::Instant;

#[test]
fn test_engine() {
    let mut engine = Engine::init();
    let fen = "rnbqkbnr/pp3ppp/2p1P3/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 4 1".to_string();
    engine.game.set_from_fen(fen.clone());
    let best_move = engine.get_best_move(1);
    let allowed_move = engine.game.play_move_ob(best_move);
    assert!(allowed_move);
}

#[test]
fn test_index_to_letters() {
    let cell = position_helper::index_to_letter(3u8);
    assert_eq!(cell, "d8");
}

#[test]
fn test_letters_to_index() {
    let cell = String::from("d8");
    let index = position_helper::letter_to_index(cell);
    assert_eq!(index, 3);
}

#[test]
fn test_pawn_initial_move_emtpy_board() {
    let board = Board::init();
    let pos_string: String = String::from("a2");
    let position = position_helper::letter_to_index(pos_string);
    let white_pawn = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + PAWN_BIT);
    let possible_positions: Vec<String> = white_pawn
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    assert_eq!(possible_positions, vec!["a3", "a4"]);
}

#[test]
fn test_pawn_cannot_take_in_front() {
    let fen_string = "rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen_string);
    let allowed_move = game.play_move_from_string("e4", "e5", "");
    assert!(!allowed_move);
}

#[test]
fn test_king_moves_empty_board() {
    let board = Board::init();
    let pos_string: String = String::from("a1");
    let position = position_helper::letter_to_index(pos_string);
    let king = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + KING);
    let mut possible_positions: Vec<String> = king
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    possible_positions.sort();
    println!(
        "The positions output for the King are: {:?}",
        possible_positions
    );
    assert_eq!(possible_positions, vec!["a2", "b1", "b2"]);
}

#[test]
fn test_rook_moves_empty_board() {
    let board = Board::init();
    let pos_string: String = String::from("d4");
    let position = position_helper::letter_to_index(pos_string.clone());
    let rook = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + ROOK);
    let possible_positions: HashSet<String> = rook
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the rook are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = [
        "a4", "b4", "c4", "d1", "d2", "d3", "d5", "d6", "d7", "d8", "e4", "f4", "g4", "h4",
    ]
    .iter()
    .map(|&x| String::from(x))
    .collect::<HashSet<String>>();
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_rook_moves_starting_board() {
    let mut board = Board::init();
    board.set_start_position();
    let pos_string: String = String::from("a1");
    let position = position_helper::letter_to_index(pos_string.clone());
    let rook = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + ROOK);
    let possible_positions: HashSet<String> = rook
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the rook are: {:?}",
        pos_string, possible_positions
    );
    // At the start of the game the rook should be blocked from moving anywhere
    assert_eq!(possible_positions.len(), 0);
}
#[test]
fn test_bishop_moves_empty_board() {
    let board = Board::init();
    let pos_string: String = String::from("d4");
    let position = position_helper::letter_to_index(pos_string.clone());
    let bishop = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + BISHOP);
    let possible_positions: HashSet<String> = bishop
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the bishop are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = [
        "a1", "a7", "b2", "b6", "c3", "c5", "e3", "e5", "f2", "f6", "g1", "g7", "h8",
    ]
    .iter()
    .map(|&x| String::from(x))
    .collect::<HashSet<String>>();
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_bishop_moves_starting_board() {
    let mut board = Board::init();
    board.set_start_position();
    let pos_string: String = String::from("c1");
    let position = position_helper::letter_to_index(pos_string.clone());
    let bishop = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + BISHOP);
    let possible_positions: HashSet<String> = bishop
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the bishop are: {:?}",
        pos_string, possible_positions
    );
    // We should have no possible moves at the beginning
    assert_eq!(possible_positions.len(), 0);
}

#[test]
fn test_queen_moves_starting_board() {
    let mut board = Board::init();
    board.set_start_position();
    let pos_string: String = String::from("c1");
    let position = position_helper::letter_to_index(pos_string.clone());
    let queen = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + QUEEN);
    let possible_positions: HashSet<String> = queen
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the queen are: {:?}",
        pos_string, possible_positions
    );
    // We should have no possible moves at the beginning
    assert_eq!(possible_positions.len(), 0);
}

#[test]
fn test_queen_moves_empty_board() {
    let board = Board::init();
    let pos_string: String = String::from("d4");
    let position = position_helper::letter_to_index(pos_string.clone());
    let queen = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + QUEEN);
    let possible_positions: HashSet<String> = queen
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the queen are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = [
        "a1", "a4", "a7", "b2", "b4", "b6", "c3", "c4", "c5", "d1", "d2", "d3", "d5", "d6", "d7",
        "d8", "e3", "e4", "e5", "f2", "f4", "f6", "g1", "g4", "g7", "h4", "h8",
    ]
    .iter()
    .map(|&x| String::from(x))
    .collect::<HashSet<String>>();
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_knight_moves_empty_board() {
    let board = Board::init();
    let pos_string: String = String::from("d4");
    let position = position_helper::letter_to_index(pos_string.clone());
    let knight = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + KNIGHT);
    let possible_positions: HashSet<String> = knight
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the knight are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = ["b3", "b5", "c2", "c6", "e2", "e6", "f3", "f5"]
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>();
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_knight_move_edge_board() {
    let board = Board::init();
    let pos_string: String = String::from("a1");
    let position = position_helper::letter_to_index(pos_string.clone());
    let knight = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + KNIGHT);
    let possible_positions: HashSet<String> = knight
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from {} for the knight are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = ["b3", "c2"]
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>();
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_validate_position_in_board() {
    let mut board = Board::init();
    board.set_start_position();
    let final_string: String = String::from("a1");
    let final_position = position_helper::letter_to_index(final_string.clone());
    let valid_position = position_helper::is_position_valid(final_position, &board, true);
    assert!(!valid_position);
}

#[test]
fn test_fen_on_start() {
    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let game = Game::init();
    let fen = game.get_fen();
    assert_eq!(fen, fen_string);
}

#[test]
fn test_update_from_fen() {
    let fen_after_e4_move =
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen_after_e4_move.clone());
    let fen2 = game.get_fen();
    assert_eq!(fen2, fen_after_e4_move);
}

#[test]
fn test_update_from_fen2() {
    let fen = "rnbqkbnr/pp3ppp/2p5/3pN3/4P3/2P5/PP1P1PPP/RNBQKB1R b KQkq - 0 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen.clone());
    let fen2 = game.get_fen();
    assert_eq!(fen2, fen);
}

#[test]
fn test_take_with_black_pawn() {
    /*
      |----|----|----|----|----|----|----|----|
    8 | bR | bN | bB | bQ | bK | bB | bN | bR |
      |----|----|----|----|----|----|----|----|
    7 | bP | bP |    |    |    | bP | bP | bP |
      |----|----|----|----|----|----|----|----|
    6 |    |    | bP |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    5 |    |    |    | bP | wN |    |    |    |
      |----|----|----|----|----|----|----|----|
    4 |    |    |    |    | wP |    |    |    |
      |----|----|----|----|----|----|----|----|
    3 |    |    | wP |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    2 | wP | wP |    | wP |    | wP | wP | wP |
      |----|----|----|----|----|----|----|----|
    1 | wR | wN | wB | wQ | wK | wB |    | wR |
      |----|----|----|----|----|----|----|----|
        a    b    c    d    e    f    g    h
     */
    let fen = "rnbqkbnr/pp3ppp/2p5/3pN3/4P3/2P5/PP1P1PPP/RNBQKB1R b KQkq - 0 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen.clone());
    let allowed_move = game.play_move_from_string("d5","e4", "");
    assert!(allowed_move);
}

#[test]
fn test_undo_move() {
    let fen = "rnbqkbnr/pp3ppp/2p5/3pN3/4P3/2P5/PP1P1PPP/RNBQKB1R b KQkq - 0 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen.clone());
    let allowed_move = game.play_move_from_string("d5", "e4", "");
    assert!(allowed_move);
    game.undo_move();
    let fen2 = game.get_fen();
    assert_eq!(fen2, fen);
}

#[test]
fn test_queen_in_position() {
    let fen = "rnbqkbnr/ppp2ppp/4P3/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 4 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen.clone());
    game.board.show();
    let allowed_move = game.play_move_from_string("d8", "d6", "");
    assert!(allowed_move);
}

#[test]
fn test_en_passant_take() {
    let mut game = Game::init();
    game.play_move_from_string("e2", "e4", "");
    game.play_move_from_string("a7", "a6", "");
    game.play_move_from_string("e4","e5", "");
    game.play_move_from_string("d7","d5","");
    let valid_move = game.play_move_from_string("e5","d6","");
    assert!(valid_move);
}

#[test]
fn test_fen_serde() {
    let fen = "rnbqkbnr/ppp2ppp/4P3/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 4 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen.clone());
    let fen2 = game.get_fen();
    assert_eq!(fen2, fen);
}

#[test]
fn test_binary_to_piece() {
    let piece = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + KING);
    assert!(piece.is_white);
    assert_eq!(piece.class, PieceType::King);
}

#[test]
fn test_binary_to_piece_queen() {
    let piece = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + QUEEN);
    assert!(piece.is_white);
    assert_eq!(piece.class, PieceType::Queen);
}

#[test]
fn test_position_start() {
    let game = Game::init();
    let black_king = game.get_piece_at_square("e8".to_string());
    let black_queen = game.get_piece_at_square("d8".to_string());
    let black_king_array = Piece::init_from_binary(game.board.state[4]);
    let black_queen_array = Piece::init_from_binary(game.board.state[3]);
    assert_eq!(black_king_array.class, PieceType::King);
    assert_eq!(black_queen_array.class, PieceType::Queen);
    assert_eq!(black_king, "k".to_string());
    assert_eq!(black_queen, "q".to_string());
}

#[test]
fn test_en_passant_flag() {
    let mut game = Game::init();
    game.play_move_from_string("e2", "e4", "");
    assert_eq!(game.en_passant, "e3");
}

//TODO: Add tests for the following:
// - Castling
// - Promotion
// - Check
// - Legal moves


#[test]
fn test_queen_moves_from_fen() {
    let mut game = Game::init();
    let fen = "rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1".to_string();
    /*
      |----|----|----|----|----|----|----|----|
    8 | bR | bN | bB | bQ | bK | bB | bN | bR |
      |----|----|----|----|----|----|----|----|
    7 | bP | bP | bP |    | bP | bP | bP | bP |
      |----|----|----|----|----|----|----|----|
    6 |    |    |    |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    5 |    |    |    | bP |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    4 |    |    |    | wP |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    3 |    |    |    |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    2 | wP | wP | wP |    | wP | wP | wP | wP |
      |----|----|----|----|----|----|----|----|
    1 | wR | wN | wB | wQ | wK | wB | wN | wR |
      |----|----|----|----|----|----|----|----|
        a    b    c    d    e    f    g    h
    */
    game.set_from_fen(fen.clone());
    let initial_position = position_helper::letter_to_index("d1".to_string());
    let white_queen_bits = game.board.state.get(initial_position as usize).unwrap();
    let queen = Piece::init_from_binary(*white_queen_bits);
    let possible_positions: HashSet<String> = queen
        .possible_moves(initial_position, &game.board)
        .iter()
        .map(|x| position_helper::index_to_letter(x.target))
        .collect();
    println!(
        "The positions from d1 for the queen are: {:?}",
        possible_positions
    );
    let correct_position: HashSet<String> = ["d2", "d3"]
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>();
    assert_eq!(PIECE_BIT + WHITE_BIT + QUEEN, *white_queen_bits);
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn update_castling_after_taken_rook() {
    /*
      |----|----|----|----|----|----|----|----|
    8 | bR | bN | bB | bQ | bK | bB |    | bR |
      |----|----|----|----|----|----|----|----|
    7 | bP | bP | bP | bP | bP | bP | bP | bP |
      |----|----|----|----|----|----|----|----|
    6 |    |    |    |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    5 |    | wN |    |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    4 |    |    |    |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    3 |    |    |    | wP |    |    | bN |    | <-- This knight can take the rook
      |----|----|----|----|----|----|----|----|
    2 | wP | wP | wP |    | wP | wP | wP | wP |
      |----|----|----|----|----|----|----|----|
    1 | wR | wN | wB | wQ | wK | wB |    | wR | <-- This rook has been taken
      |----|----|----|----|----|----|----|----|
        a    b    c    d    e    f    g    h
     */

    let mut game = Game::init();
    game.set_from_fen("rnbqkb1r/pppppppp/8/8/3N4/2PP2n1/PP2PPPP/RNBQKB1R b KQkq - 0 4".to_string());
    let allowed_move = game.play_move_from_string("g3", "h1", "");
    assert!(allowed_move);

    // Check that the castling rights have been updated - the white king should not be able to castle on the kingside
    assert_eq!(game.board.castling, 0b00000111);
}

#[test]
fn test_legal_moves_should_allow_taking_piece_to_avoid_check() {
    /*
      |----|----|----|----|----|----|----|----|
    8 | bR | bN | bB |    | bK | bB | bN | bR |
      |----|----|----|----|----|----|----|----|
    7 | bP | bP | bP |    | bP | bP | bP | bP |
      |----|----|----|----|----|----|----|----|
    6 |    |    |    |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    5 |    |    |    | wP |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    4 |    |    |    |    |    |    |    |    |
      |----|----|----|----|----|----|----|----|
    3 |    |    |    |    | bQ |    |    |    | <-- bQ is checking the white king
      |----|----|----|----|----|----|----|----|
    2 | wP | wP |    |    |    | wP | wP | wP | <-- This pawn can take the black queen
      |----|----|----|----|----|----|----|----|
    1 | wR | wN | wB | wQ | wK | wB | wN | wR | <-- bishops, queen and knight should be allowed to move if they block the king
      |----|----|----|----|----|----|----|----|
        a    b    c    d    e    f    g    h

     */
    let mut game = Game::init();
    game.set_from_fen("rnb1kbnr/ppp1pppp/8/3P4/8/4q3/PP3PPP/RNBQKBNR w KQkq - 0 7".to_string());
    let moves = game.get_legal_moves(game.white_turn);
    for mv in moves.clone() {
      let source = position_helper::index_to_letter(mv.source);
      let target = position_helper::index_to_letter(mv.target);
      println!("source: {source} to {target}");
    }
    assert_eq!(moves.len(), 5); // take with bishop, take with pawn, block with queen, block with knight, block with bishop
}

#[test]
fn test_legal_move_generation() {
    let mut new_game = Game::init();
    let start = Instant::now();
    let moves = perft(1, &mut new_game);
    let elapsed = start.elapsed();
    println!("Time taken for depth 1: {:?}", elapsed);
    
    let start = Instant::now();
    let moves2 = perft(2, &mut new_game);
    let elapsed = start.elapsed();
    println!("Time taken for depth 2: {:?}", elapsed);
    
    let start = Instant::now();
    let moves3 = perft(3, &mut new_game);
    let elapsed = start.elapsed();
    println!("Time taken for depth 3: {:?}", elapsed);
    
    let start = Instant::now();
    let moves4 = perft(4, &mut new_game);
    let elapsed = start.elapsed();
    println!("Time taken for depth 4: {:?}", elapsed);
    assert_eq!(moves, 20);
    assert_eq!(moves2, 400);
    assert_eq!(moves3, 8902);
    assert_eq!(moves4, 197281);
}

#[test]
fn test_legal_move_generation_postion1() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    let start = Instant::now();
    let moves = perft(1, &mut game);
    let elapsed = start.elapsed();
    println!("Time taken for depth 1: {:?}", elapsed);

    let start = Instant::now();
    let moves2 = perft(2, &mut game);
    let elapsed = start.elapsed();
    println!("Time taken for depth 2: {:?}", elapsed);

    let start = Instant::now();
    let moves3 = perft(3, &mut game);
    let elapsed = start.elapsed();
    println!("Time taken for depth 3: {:?}", elapsed);

    // let start = Instant::now();
    // let moves4 = perft(4, &mut game);
    // let elapsed = start.elapsed();
    // println!("Time taken for depth 4: {:?}", elapsed);

    assert_eq!(moves, 48);
    assert_eq!(moves2, 2039);
    assert_eq!(moves3, 97862);
    // assert_eq!(moves4, 4085603);

} 

fn perft(depth: u8, game: &mut Game) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut count = 0;

    let moves = game.get_all_moves_bitboard(game.white_turn);
    if depth == 1 {
      return moves.len();
    }

    for mv in moves {
        // Use play_move with legality check disabled since moves are already legal
        let success = game.play_move(mv, false);
        if success {
            count += perft(depth - 1, game);
            game.undo_move();
        }
    }
    count
}

#[test]
fn check_duplicate_moves() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    let moves = game.get_legal_moves(true);
    let mut move_strings = Vec::new();
    let mut seen_moves = std::collections::HashSet::new();
    let mut duplicates = Vec::new();
    
    for mv in &moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        let move_str = format!("{}-{}-{}", source, target, mv.promotion);
        
        if !seen_moves.insert(move_str.clone()) {
            duplicates.push(move_str.clone());
        }
        move_strings.push(move_str);
    }
    
    if !duplicates.is_empty() {
        println!("Found {} duplicate moves:", duplicates.len());
        for dup in duplicates {
            println!("  {}", dup);
        }
    } else {
        println!("No duplicate moves found at depth 1");
    }
    
    println!("Total unique moves: {}", seen_moves.len());
    println!("Total moves: {}", moves.len());
}

#[test]
fn check_undo_functionality() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    let original_fen = game.get_fen();
    let moves = game.get_legal_moves(true);
    
    println!("Testing undo functionality with {} moves", moves.len());
    let mut failed_undos = 0;
    
    for (i, mv) in moves.iter().enumerate() {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        
        // Play the move
        let success = game.play_move_ob(*mv);
        if !success {
            println!("Move {} ({} to {}) failed to play", i, source, target);
            continue;
        }
        
        // Undo the move
        game.undo_move();
        
        // Check if we're back to original state
        let restored_fen = game.get_fen();
        if restored_fen != original_fen {
            failed_undos += 1;
            println!("Undo failed for move {} ({} to {})", i, source, target);
            println!("Original:  {}", original_fen);
            println!("Restored:  {}", restored_fen);
            
            // Restore original position for next test
            game.set_from_fen(original_fen.clone());
        }
    }
    
    if failed_undos == 0 {
        println!("All undo operations successful!");
    } else {
        println!("{} undo operations failed", failed_undos);
    }
}

#[test]
fn check_castling_legality() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    println!("Checking castling moves legality");
    
    // Find king moves
    let moves = game.get_legal_moves(true);
    let king_moves: Vec<_> = moves.iter().filter(|mv| {
        let piece = game.board.state[mv.source as usize];
        let piece_obj = Piece::init_from_binary(piece);
        piece_obj.class == PieceType::King
    }).collect();
    
    println!("King has {} legal moves:", king_moves.len());
    for mv in king_moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        let move_distance = (mv.target as i8 - mv.source as i8).abs();
        let is_castling = move_distance == 2;
        println!("  {} to {} (distance: {}, castling: {})", source, target, move_distance, is_castling);
        
        if is_castling {
            // Test this castling move in isolation
            game.play_move_ob(*mv);
            println!("    After castling: {}", game.get_fen_simple());
            
            // Check opponent responses
            let opponent_moves = game.get_legal_moves(false);
            println!("    Opponent has {} responses", opponent_moves.len());
            
            game.undo_move();
        }
    }
}

#[test]
fn detailed_perft_position1() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    let depth1 = perft(1, &mut game);
    let depth2 = perft(2, &mut game);
    let depth3 = perft(3, &mut game);
    
    println!("Perft results for position 1:");
    println!("Depth 1: {} (expected: 48)", depth1);
    println!("Depth 2: {} (expected: 2039)", depth2);
    println!("Depth 3: {} (expected: 97862)", depth3);
    
    println!("Differences:");
    println!("Depth 1: {}", depth1 - 48);
    println!("Depth 2: {}", depth2 - 2039);  
    println!("Depth 3: {}", depth3 - 97862);
}

#[test]
fn test_castling_through_check() {
    // Create a position where castling would move through check
    let mut game = Game::init();
    // This position has a black queen on f3 that attacks f1 (kingside castling path)
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    println!("Testing castling legality in problematic position");
    
    let moves = game.get_legal_moves(true);
    let king_moves: Vec<_> = moves.iter().filter(|mv| {
        let piece = game.board.state[mv.source as usize];
        let piece_obj = Piece::init_from_binary(piece);
        piece_obj.class == PieceType::King
    }).collect();
    
    for mv in king_moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        let move_distance = (mv.target as i8 - mv.source as i8).abs();
        let is_castling = move_distance == 2;
        
        if is_castling {
            println!("Found castling move: {} to {}", source, target);
            
            // Manually check if this castling move is through check
            if target == "g1" {
                // Kingside castling - check if f1 is attacked
                println!("Checking if f1 is attacked by black pieces");
                let f1_index = position_helper::letter_to_index("f1".to_string());
                let black_moves = game.get_all_moves_for_color(false);
                let f1_attacked = black_moves.iter().any(|mv| mv.target == f1_index);
                println!("f1 attacked: {}", f1_attacked);
                
                if f1_attacked {
                    println!("ERROR: Castling through check should be illegal!");
                }
            }
            
            if target == "c1" {
                // Queenside castling - check if d1 is attacked
                println!("Checking if d1 is attacked by black pieces");
                let d1_index = position_helper::letter_to_index("d1".to_string());
                let black_moves = game.get_all_moves_for_color(false);
                let d1_attacked = black_moves.iter().any(|mv| mv.target == d1_index);
                println!("d1 attacked: {}", d1_attacked);
                
                if d1_attacked {
                    println!("ERROR: Castling through check should be illegal!");
                }
            }
        }
    }
}

#[test]
fn check_en_passant_in_position1() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    println!("En passant square from FEN: {}", game.en_passant);
    println!("Board en passant: {}", game.board.en_passant);
    
    // Check if there are any pawn moves that might create en passant opportunities
    let moves = game.get_legal_moves(true);
    let pawn_moves: Vec<_> = moves.iter().filter(|mv| {
        let piece = game.board.state[mv.source as usize];
        let piece_obj = Piece::init_from_binary(piece);
        piece_obj.class == PieceType::Pawn
    }).collect();
    
    println!("Found {} pawn moves", pawn_moves.len());
    for mv in pawn_moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        println!("  {} to {}", source, target);
        
        // Check if this creates en passant opportunity
        let source_row = position_helper::get_row(mv.source);
        let target_row = position_helper::get_row(mv.target);
        let row_diff = (source_row as i8 - target_row as i8).abs();
        
        if row_diff == 2 {
            println!("    -> This is a double pawn move (creates en passant)");
        }
    }
}

#[test]  
fn check_move_validation_consistency() {
    // Check if pseudolegal vs legal move counts are consistent
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    let pseudo_moves = game.get_all_moves_for_color(true);
    let legal_moves = game.get_legal_moves(true);
    
    println!("Pseudolegal moves: {}", pseudo_moves.len());
    println!("Legal moves: {}", legal_moves.len());
    println!("Filtered out: {}", pseudo_moves.len() - legal_moves.len());
    
    // Check for any inconsistencies in the filtering process
    let mut inconsistent_moves = 0;
    
    for pseudo_mv in &pseudo_moves {
        let is_in_legal = legal_moves.iter().any(|legal_mv| {
            legal_mv.source == pseudo_mv.source && 
            legal_mv.target == pseudo_mv.target && 
            legal_mv.promotion == pseudo_mv.promotion
        });
        
        if !is_in_legal {
            // This pseudolegal move was filtered out - let's verify it's actually illegal
            let mut test_game = game.clone();
            let success = test_game.play_move_ob(*pseudo_mv);
            
            if success {
                // Move played successfully - check if king is in check
                let king_pos = test_game.board.get_king_position(!test_game.white_turn);
                let opponent_moves = test_game.get_all_moves_for_color(test_game.white_turn);
                let king_in_check = opponent_moves.iter().any(|mv| mv.target == king_pos);
                
                if !king_in_check {
                    inconsistent_moves += 1;
                    let source = position_helper::index_to_letter(pseudo_mv.source);
                    let target = position_helper::index_to_letter(pseudo_mv.target);
                    println!("INCONSISTENCY: {} to {} should be legal but was filtered", source, target);
                }
            }
        }
    }
    
    if inconsistent_moves == 0 {
        println!("Move validation is consistent");
    } else {
        println!("Found {} inconsistent moves", inconsistent_moves);
    }
}

#[test]
fn debug_perft_position1() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    let moves = game.get_legal_moves(true);
    println!("Total moves at depth 1: {}", moves.len());
    
    // Now let's check depth 2 and identify problematic moves
    let mut total_depth2 = 0;
    let mut move_analysis = Vec::new();
    
    for mv in moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        
        game.play_move_ob(mv);
        let depth2_moves = perft(1, &mut game);
        total_depth2 += depth2_moves;
        
        move_analysis.push((source.clone(), target.clone(), depth2_moves));
        game.undo_move();
    }
    
    // Sort by move count to find anomalies
    move_analysis.sort_by(|a, b| b.2.cmp(&a.2));
    
    println!("Top moves by response count:");
    for (source, target, count) in move_analysis.iter().take(10) {
        println!("{} to {}: {} responses", source, target, count);
    }
    
    println!("Total moves at depth 2: {}", total_depth2);
    println!("Expected: 2039, Got: {}", total_depth2);
    
    // Let's also check if any specific move types have issues
    let expected_avg = 2039.0 / 48.0; // Expected average responses per move
    println!("Expected average responses per move: {:.1}", expected_avg);
    
    for (source, target, count) in move_analysis.iter() {
        if *count as f32 > expected_avg + 5.0 {
            println!("Anomaly: {} to {} has {} responses (much higher than average)", source, target, count);
        }
    }
}

#[test]
fn test_promotion() {
  let mut game = Game::init();
  game.set_from_simple_fen("8/P7/8/8/8/8/8/8".to_string());
  let mv = Move{
    source: 8u8,
    target: 0,
    promotion: PIECE_BIT + WHITE_BIT + QUEEN,
  };
  let success = game.play_move(mv, false);
  
  assert!(success);

  let pawn_should_be_queen = Piece::init_from_binary(game.board.state[0]);
  assert_eq!(pawn_should_be_queen.class, PieceType::Queen);

  // Promote to another piece
  game.undo_move();
  let mv = Move { source: 8u8, target: 0, promotion: PIECE_BIT + WHITE_BIT + KNIGHT };
  game.play_move(mv, false);
  let knight_piece = Piece::init_from_binary(game.board.state[0]);
  assert_eq!(knight_piece.class, PieceType::Knight);
}

#[test]
fn test_pawn_move_gen_speed() {
  let mut game = Game::init();
  game.set_from_simple_fen("8/P7/8/8/8/8/8/8".to_string());
  let start = Instant::now();
  for _ in 0..100 {
    game.get_legal_moves(true);
  }
  println!("Pawn move gen took {:?}", start.elapsed());
}

#[test]
fn test_rook_move_gen_speed() {
  let mut game = Game::init();
  game.set_from_simple_fen("8/R7/8/8/8/8/8/8".to_string());
  let start = Instant::now();
  for _ in 0..100 {
    game.get_legal_moves(true);
  }
  println!("Pawn move gen took {:?}", start.elapsed());
}

#[test]
fn test_king_move_gen_speed() {
  let mut game = Game::init();
  game.set_from_simple_fen("8/K7/8/8/8/8/8/8".to_string());
  let start = Instant::now();
  for _ in 0..100 {
    game.get_legal_moves(true);
  }
  println!("Pawn move gen took {:?}", start.elapsed());
}

#[test]
fn test_knight_move_gen_speed() {
  let mut game = Game::init();
  game.set_from_simple_fen("8/N7/8/8/8/8/8/8".to_string());
  let start = Instant::now();
  for _ in 0..100 {
    game.get_legal_moves(true);
  }
  println!("Pawn move gen took {:?}", start.elapsed());
}

#[test]
fn test_queen_move_gen_speed() {
  let mut game = Game::init();
  game.set_from_simple_fen("8/Q7/8/8/8/8/8/8".to_string());
  let start = Instant::now();
  for _ in 0..100 {
    game.get_legal_moves(true);
  }
  println!("Pawn move gen took {:?}", start.elapsed());
}

#[test]
fn investigate_high_response_moves() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    // Test the move d5 to e6 that had 46 responses
    let d5_index = position_helper::letter_to_index("d5".to_string());
    let e6_index = position_helper::letter_to_index("e6".to_string());
    let mv = Move {
        source: d5_index,
        target: e6_index,
        promotion: 0,
    };
    
    println!("Playing d5 to e6...");
    let success = game.play_move_ob(mv);
    assert!(success);
    
    // Count legal moves for black
    let black_moves = game.get_legal_moves(false); // Black to move
    println!("Black has {} legal moves after d5 to e6", black_moves.len());
    
    // Check for duplicate moves at this position
    let mut move_strings = std::collections::HashSet::new();
    let mut duplicates = 0;
    
    for mv in &black_moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        let move_str = format!("{}-{}-{}", source, target, mv.promotion);
        
        if !move_strings.insert(move_str.clone()) {
            duplicates += 1;
            println!("DUPLICATE: {}", move_str);
        }
    }
    
    if duplicates > 0 {
        println!("Found {} duplicate moves after d5 to e6", duplicates);
    }
    
    game.undo_move();
    
    // Test the move e5 to d7 that also had 46 responses
    let e5_index = position_helper::letter_to_index("e5".to_string());
    let d7_index = position_helper::letter_to_index("d7".to_string());
    let mv2 = Move {
        source: e5_index,
        target: d7_index,
        promotion: 0,
    };
    
    println!("Playing e5 to d7...");
    let success2 = game.play_move_ob(mv2);
    assert!(success2);
    
    // Count legal moves for black
    let black_moves2 = game.get_legal_moves(false); // Black to move
    println!("Black has {} legal moves after e5 to d7", black_moves2.len());
    
    // Check for duplicate moves at this position
    let mut move_strings2 = std::collections::HashSet::new();
    let mut duplicates2 = 0;
    
    for mv in &black_moves2 {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        let move_str = format!("{}-{}-{}", source, target, mv.promotion);
        
        if !move_strings2.insert(move_str.clone()) {
            duplicates2 += 1;
            println!("DUPLICATE: {}", move_str);
        }
    }
    
    if duplicates2 > 0 {
        println!("Found {} duplicate moves after e5 to d7", duplicates2);
    }
}

#[test]
fn find_problematic_moves() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    let moves = game.get_legal_moves(true);
    
    // Check each move and compare depth counts with a reference engine if needed
    let mut high_response_moves = Vec::new();
    
    for mv in moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        
        game.play_move_ob(mv);
        let responses = game.get_legal_moves(false).len();
        
        if responses > 45 { // Flagging unusually high response counts
            high_response_moves.push((source.clone(), target.clone(), responses));
        }
        
        game.undo_move();
    }
    
    println!("Found {} moves with >45 responses:", high_response_moves.len());
    for (source, target, count) in high_response_moves {
        println!("  {} to {}: {} responses", source, target, count);
    }
}

#[test]
fn detailed_knight_analysis() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    // Focus on the knight move d5 to e6
    let d5_index = position_helper::letter_to_index("d5".to_string());
    let e6_index = position_helper::letter_to_index("e6".to_string());
    let mv = Move {
        source: d5_index,
        target: e6_index,
        promotion: 0,
    };
    
    // Get the original position
    let original_fen = game.get_fen();
    println!("Original position: {}", original_fen);
    
    // Play the move
    game.play_move_ob(mv);
    let after_move_fen = game.get_fen();
    println!("After d5xe6: {}", after_move_fen);
    
    // Show the board state
    println!("Board after knight capture:");
    game.board.show();
    
    // Count responses
    let black_responses = game.get_legal_moves(false);
    println!("Black has {} legal responses", black_responses.len());
    
    // Check a few specific black moves to see if they generate the right number of white responses
    let mut total_white_responses = 0;
    for (_i, black_mv) in black_responses.iter().enumerate().take(5) {
        let b_source = position_helper::index_to_letter(black_mv.source);
        let b_target = position_helper::index_to_letter(black_mv.target);
        
        game.play_move_ob(*black_mv);
        let white_moves = game.get_legal_moves(true);
        let white_responses = white_moves.len();
        total_white_responses += white_responses;
        
        // Check for duplicates in white responses
        let mut white_move_strings = std::collections::HashSet::new();
        let mut white_duplicates = 0;
        
        for w_mv in &white_moves {
            let w_source = position_helper::index_to_letter(w_mv.source);
            let w_target = position_helper::index_to_letter(w_mv.target);
            let w_move_str = format!("{}-{}-{}", w_source, w_target, w_mv.promotion);
            
            if !white_move_strings.insert(w_move_str.clone()) {
                white_duplicates += 1;
                println!("    DUPLICATE WHITE MOVE: {}", w_move_str);
            }
        }
        
        println!("  Black {} to {}: {} white responses ({} duplicates)", b_source, b_target, white_responses, white_duplicates);
        game.undo_move();
    }
    
    println!("First 5 black moves generate {} total white responses", total_white_responses);
    
    // Now undo and check the alternative expected behavior
    game.undo_move();
    let restored_fen = game.get_fen();
    println!("Restored to: {}", restored_fen);
    assert_eq!(restored_fen, original_fen, "Undo failed!");
}

#[test]
fn analyze_white_piece_moves_after_capture() {
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    // Play d5xe6
    let d5_index = position_helper::letter_to_index("d5".to_string());
    let e6_index = position_helper::letter_to_index("e6".to_string());
    let mv = Move { source: d5_index, target: e6_index, promotion: 0 };
    game.play_move_ob(mv);
    
    // Play a simple black rook move (a8-b8)
    let a8_index = position_helper::letter_to_index("a8".to_string());
    let b8_index = position_helper::letter_to_index("b8".to_string());
    let black_mv = Move { source: a8_index, target: b8_index, promotion: 0 };
    game.play_move_ob(black_mv);
    
    println!("Position after d5xe6, Ra8-b8:");
    println!("FEN: {}", game.get_fen());
    game.board.show();
    
    // Analyze moves by piece type
    let all_white_moves = game.get_all_moves_for_color(true);
    println!("Total pseudolegal white moves: {}", all_white_moves.len());
    
    let legal_white_moves = game.get_legal_moves(true);
    println!("Total legal white moves: {}", legal_white_moves.len());
    
    // Count moves by piece type
    let mut piece_move_counts = std::collections::HashMap::new();
    for mv in &legal_white_moves {
        let piece_byte = game.board.state[mv.source as usize];
        let piece = Piece::init_from_binary(piece_byte);
        let piece_name = format!("{:?}", piece.class);
        
        *piece_move_counts.entry(piece_name).or_insert(0) += 1;
    }
    
    println!("Legal moves by piece type:");
    for (piece_type, count) in piece_move_counts {
        println!("  {}: {} moves", piece_type, count);
    }
    
    // Check for specific pieces that might be problematic
    println!("\nDetailed analysis of specific pieces:");
    
    // Check pawn on e6
    let e6_piece = game.board.state[e6_index as usize];
    if e6_piece != 0 {
        let piece = Piece::init_from_binary(e6_piece);
        let pawn_moves = piece.possible_moves(e6_index, &game.board);
        println!("Pawn on e6: {} possible moves", pawn_moves.len());
        for mv in pawn_moves {
            let target = position_helper::index_to_letter(mv.target);
            println!("  e6 to {}", target);
        }
    }
    
    // Check knight on e5
    let e5_index = position_helper::letter_to_index("e5".to_string());
    let e5_piece = game.board.state[e5_index as usize];
    if e5_piece != 0 {
        let piece = Piece::init_from_binary(e5_piece);
        let knight_moves = piece.possible_moves(e5_index, &game.board);
        println!("Knight on e5: {} possible moves", knight_moves.len());
        for mv in knight_moves {
            let target = position_helper::index_to_letter(mv.target);
            println!("  e5 to {}", target);
        }
    }
}

#[test]
fn validate_specific_position_moves() {
    // Let's manually validate some moves in the problematic position
    let mut game = Game::init();
    game.set_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());
    
    // Play d5xe6
    let d5_index = position_helper::letter_to_index("d5".to_string());
    let e6_index = position_helper::letter_to_index("e6".to_string());
    let mv = Move { source: d5_index, target: e6_index, promotion: 0 };
    game.play_move_ob(mv);
    
    // Play a simple black rook move (a8-b8)
    let a8_index = position_helper::letter_to_index("a8".to_string());
    let b8_index = position_helper::letter_to_index("b8".to_string());
    let black_mv = Move { source: a8_index, target: b8_index, promotion: 0 };
    game.play_move_ob(black_mv);
    
    println!("Validating all white moves for legality...");
    let legal_white_moves = game.get_legal_moves(true);
    
    let mut validation_issues = 0;
    for mv in legal_white_moves {
        let source = position_helper::index_to_letter(mv.source);
        let target = position_helper::index_to_letter(mv.target);
        
        // Try to play each move and see if it results in a valid position
        let mut test_game = game.clone();
        let success = test_game.play_move_ob(mv);
        
        if !success {
            validation_issues += 1;
            println!("INVALID MOVE: {} to {} (move failed to play)", source, target);
            continue;
        }
        
        // Check if our own king is left in check after this move
        let our_king_pos = test_game.board.get_king_position(!test_game.white_turn);
        let opponent_moves = test_game.get_all_moves_for_color(test_game.white_turn);
        let king_in_check = opponent_moves.iter().any(|opp_mv| opp_mv.target == our_king_pos);
        
        if king_in_check {
            validation_issues += 1;
            println!("INVALID MOVE: {} to {} (leaves king in check)", source, target);
        }
    }
    
    if validation_issues == 0 {
        println!("All moves are valid!");
    } else {
        println!("Found {} invalid moves!", validation_issues);
    }
}
