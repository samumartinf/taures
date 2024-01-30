use std::collections::HashSet;
use cherris::*;
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

#[test]
fn test_index_to_letters() {
    let pos_byte = position_helper::index_to_position_byte(3); // 3 = Black Queen
    let cell = position_helper::position_byte_to_letter(pos_byte);
    assert_eq!(pos_byte, 0b00000011);
    assert_eq!(cell, "d8");
}

#[test]
fn test_letters_to_index() {
    let cell = String::from("d8");
    let pos_byte = position_helper::letter_to_position_byte(cell);
    println!("The position byte returned is {}", pos_byte);
    let index = position_helper::position_byte_to_index(pos_byte);
    assert_eq!(pos_byte, 0b00000011);
    assert_eq!(index, 3);
}

#[test]
fn test_state_pieces_parity() {
    let mut board = Board::init();
    board.update_hashmap();
    let piece = *board.pieces.get(&0b00000011).unwrap();
    assert_eq!(piece, PIECE_BIT + QUEEN); // Black queen should be on index 3 after init()
}

#[test]
fn test_pawn_initial_move_emtpy_board() {
    let board = Board::init();
    let pos_string: String = String::from("a2");
    let position = position_helper::letter_to_position_byte(pos_string);
    let white_pawn = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + PAWN_BIT);
    let possible_positions: Vec<String> = white_pawn
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
        .collect();
    assert_eq!(possible_positions, vec!["a3", "a4"]);
}

#[test]
fn test_king_moves_empty_board() {
    let board = Board::init();
    let pos_string: String = String::from("a1");
    let position = position_helper::letter_to_position_byte(pos_string);
    let king = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + KING);
    let mut possible_positions: Vec<String> = king
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
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
    let position = position_helper::letter_to_position_byte(pos_string.clone());
    let rook = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + ROOK);
    let possible_positions: HashSet<String> = rook
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
        .collect();
    println!(
        "The positions from {} for the rook are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = HashSet::from(
        [
            "a4", "b4", "c4", "d1", "d2", "d3", "d5", "d6", "d7", "d8", "e4", "f4", "g4", "h4",
        ]
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>(),
    );
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_rook_moves_starting_board() {
    let mut board = Board::init();
    board.update_hashmap();
    let pos_string: String = String::from("a1");
    let position = position_helper::letter_to_position_byte(pos_string.clone());
    let rook = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + ROOK);
    let possible_positions: HashSet<String> = rook
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
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
    let position = position_helper::letter_to_position_byte(pos_string.clone());
    let bishop = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + BISHOP);
    let possible_positions: HashSet<String> = bishop
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
        .collect();
    println!(
        "The positions from {} for the bishop are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = HashSet::from(
        [
            "a1", "a7", "b2", "b6", "c3", "c5", "e3", "e5", "f2", "f6", "g1", "g7", "h8",
        ]
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>(),
    );
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_bishop_moves_starting_board() {
    let mut board = Board::init();
    board.update_hashmap();
    let pos_string: String = String::from("c1");
    let position = position_helper::letter_to_position_byte(pos_string.clone());
    let bishop = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + BISHOP);
    let possible_positions: HashSet<String> = bishop
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
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
    board.update_hashmap();
    let pos_string: String = String::from("c1");
    let position = position_helper::letter_to_position_byte(pos_string.clone());
    let queen = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + QUEEN);
    let possible_positions: HashSet<String> = queen 
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
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
    let position = position_helper::letter_to_position_byte(pos_string.clone());
    let queen = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + QUEEN);
    let possible_positions: HashSet<String> = queen
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
        .collect();
    println!(
        "The positions from {} for the queen are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = HashSet::from(
        [
            "a1", "a4", "a7", "b2", "b4", "b6", "c3", "c4", "c5", "d1", "d2", "d3", "d5", "d6",
            "d7", "d8", "e3", "e4", "e5", "f2", "f4", "f6", "g1", "g4", "g7", "h4", "h8",
        ]
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>(),
    );
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_knight_moves_empty_board() {
    let board = Board::init();
    let pos_string: String = String::from("d4");
    let position = position_helper::letter_to_position_byte(pos_string.clone());
    let knight = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + KNIGHT);
    let possible_positions: HashSet<String> = knight
        .possible_moves(position, &board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
        .collect();
    println!(
        "The positions from {} for the knight are: {:?}",
        pos_string, possible_positions
    );
    let correct_position: HashSet<String> = HashSet::from(
        ["b3", "b5", "c2", "c6", "e2", "e6", "f3", "f5"]
            .iter()
            .map(|&x| String::from(x))
            .collect::<HashSet<String>>(),
    );
    assert_eq!(possible_positions, correct_position);
}

#[test]
fn test_validate_position_in_board() {
    let mut board = Board::init();
    board.update_hashmap();
    let final_string: String = String::from("a1");
    let final_position = position_helper::letter_to_position_byte(final_string.clone());
    let valid_position = position_helper::is_position_valid(final_position, &board, true);
    assert_eq!(valid_position, false);
}
