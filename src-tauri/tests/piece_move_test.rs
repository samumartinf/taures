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
fn test_start_position_array_to_hashmap() {
    let mut board = Board::init();
    board.set_start_position();
    let piece = *board.pieces.get(&0b00000011).unwrap();
    assert_eq!(piece, PIECE_BIT + QUEEN); // Black queen should be on index 3 after init()
}

#[test]
fn test_pawn_initial_move_emtpy_board() {
    let mut board = Board::init();
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
fn test_pawn_cannot_take_in_front() {
    let fen_string = "rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen_string);
    let allowed_move = game.play_move_from_string("e4".to_string(), "e5".to_string());
    assert_eq!(allowed_move, false);
}

#[test]
fn test_king_moves_empty_board() {
    let mut board = Board::init();
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
    let mut board = Board::init();
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
    board.set_start_position();
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
    board.set_start_position();
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
    board.set_start_position();
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
    board.set_start_position();
    let final_string: String = String::from("a1");
    let final_position = position_helper::letter_to_position_byte(final_string.clone());
    let valid_position = position_helper::is_position_valid(final_position, &board, true);
    assert_eq!(valid_position, false);
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
    let allowed_move = game.play_move_from_string("d5".to_string(), "e4".to_string());
    assert_eq!(allowed_move, true);
}

#[test]
fn test_undo_move() {
    let fen = "rnbqkbnr/pp3ppp/2p5/3pN3/4P3/2P5/PP1P1PPP/RNBQKB1R b KQkq - 0 1".to_string();
    let mut game = Game::init();
    game.set_from_fen(fen.clone());
    let allowed_move = game.play_move_from_string("d5".to_string(), "e4".to_string());
    assert_eq!(allowed_move, true);
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
    let allowed_move = game.play_move_from_string("d8".to_string(), "d6".to_string());
    assert_eq!(allowed_move, true);
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
    assert_eq!(piece.is_white, true);
    assert_eq!(piece.class, PieceType::King);
}

#[test]
fn test_binary_to_piece_queen() {
    let piece = Piece::init_from_binary(PIECE_BIT + WHITE_BIT + QUEEN);
    assert_eq!(piece.is_white, true);
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
fn test_hashmap_array_parity() {
    let mut game = Game::init();
    let fen = "rnbqkbnr/pp3ppp/2p5/3pN3/4P3/2P5/PP1P1PPP/RNBQKB1R b KQkq - 0 1".to_string();
    game.set_from_fen(fen.clone());
    let mut parity = true;
    for (key, value) in game.board.pieces.iter() {
        let piece = *value;
        let piece_from_board = *game.board.pieces.get(&key).unwrap();
        if piece != piece_from_board {
            parity = false;
        }
    }
    assert_eq!(parity, true);
}

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
    let initial_position = position_helper::letter_to_position_byte("d1".to_string());
    let white_queen_bits = game.board.pieces.get(&initial_position).unwrap();
    let queen = Piece::init_from_binary(*white_queen_bits);
    let possible_positions: HashSet<String> = queen
        .possible_moves(initial_position, &game.board)
        .iter()
        .map(|x| position_helper::position_byte_to_letter(*x))
        .collect();
    println!(
        "The positions from {} for the queen are: {:?}",
        "d1", possible_positions
    );
    let correct_position: HashSet<String> = HashSet::from(
        [
            "d2", "d3"
        ]
        .iter()
        .map(|&x| String::from(x))
        .collect::<HashSet<String>>(),
    );
    assert_eq!(PIECE_BIT+WHITE_BIT+QUEEN, *white_queen_bits);
    assert_eq!(possible_positions, correct_position);
}
