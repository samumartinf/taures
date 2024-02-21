use std::vec;

pub mod board;
pub mod piece;

use board::Board;
use piece::{BasicPiece, Piece, PieceType};
const PIECE_BIT: u8 = 128u8;
const WHITE_BIT: u8 = 64u8;
const PAWN_BIT: u8 = 8u8;
const CHECK_PIECE: u8 = 0b00001111;
const KING: u8 = 0u8;
const QUEEN: u8 = 1u8;
const BISHOP: u8 = 2u8;
const KNIGHT: u8 = 4u8;
const ROOK: u8 = 6u8;
const ROW: u8 = 8u8;
const COL: u8 = 1u8;

#[derive(Debug, Clone)]
/// Represents a game of chess.
pub struct Game {
    /// Indicates whether it is currently white's turn to move.
    pub white_turn: bool,
    /// Represents the previous FEN positions of the game.
    previous_fen_positions: Vec<String>,
    /// Represents the chess board.
    pub board: Board,
    /// Indicates whether the game is done.
    game_done: bool,
    /// Represents the en passant square, if any. The options are:
    /// - A square (i.e. "e3")
    /// - A dash ("-") if there is no en passant square
    pub en_passant: String,
    half_move_clock: i32,
    full_move_number: i32,
}

pub trait ChessGame {
    fn play_move_from_string(&mut self, initial_position: String, final_position: String) -> bool;
    fn play_move(&mut self, initial_position: u8, final_position: u8) -> bool;
    fn play_move_ob(&mut self, chess_move: &Move) -> bool;
    fn get_fen(&self) -> String;
    fn set_from_fen(&mut self, fen: String);
    fn get_fen_simple(&self) -> String;
    fn restart(&mut self);
    fn undo_move(&mut self);
    fn get_pseudolegal_moves(&self, position: String) -> Vec<String>;
    fn get_all_moves_for_color(&self, white: bool) -> Vec<Move>;
    fn get_capture_moves(&self) -> Vec<Move>;
    fn get_legal_moves(&self, white: bool) -> Vec<Move>;
    fn play_legal_move(&mut self, mv: &Move) -> bool;
}

pub trait ChessDebugInfo {
    fn get_piece_at_square(&self, square: String) -> String;
}

impl ChessDebugInfo for Game {
    fn get_piece_at_square(&self, square: String) -> String {
        let index = position_helper::letter_to_index(square);
        let piece_byte = self.board.state.get(index as usize);
        if let Some(piece_byte) = piece_byte {
            let piece = Piece::init_from_binary(*piece_byte);
            return piece.fen_repr();
        }
        String::from("None")
    }
}

impl Game {
    pub fn init() -> Game {
        let mut board = Board::init();
        board.set_start_position();
        Game {
            white_turn: true,
            previous_fen_positions: vec![],
            board,
            game_done: false,
            en_passant: "-".to_string(),
            half_move_clock: 0i32,
            full_move_number: 1i32,
        }
    }

    pub fn show(&self) {
        self.board.show();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub source: u8,    // source position byte
    pub target: u8,    // target position byte
    pub promotion: u8, // piece to promote to
}

impl ChessGame for Game {
    fn get_legal_moves(&self, white: bool) -> Vec<Move> {
        // get my king position
        let mut king_position = 0;
        for i in 0..63 {
            let piece = self.board.state[i];
            if piece == 0 {
                continue;
            }
            let piece = Piece::init_from_binary(piece);
            if piece.class == PieceType::King && piece.is_white == white {
                king_position = i;
                break;
            }
        }

        let mut game_copy = self.clone();
        // define the filter function
        let mut final_moves: Vec<Move> = vec![];
        for mv in self.get_all_moves_for_color(white) {
            let success = game_copy.play_move_ob(&mv.clone());

            // if the move is illegal, skip it
            if !success {
                continue;
            }
            let oponent_moves = self.get_all_moves_for_color(!self.white_turn);
            let mut king_in_check = false;

            // check if the king is in check
            for oponent_move in oponent_moves {
                if oponent_move.target == king_position as u8 {
                    game_copy.undo_move();
                    king_in_check = true;
                    break;
                }
            }

            // if the king is not in check, add the move to the final moves
            if !king_in_check {
                final_moves.push(mv);
            }
            game_copy.undo_move();
        }

        final_moves
    }

    fn play_legal_move(&mut self, mv: &Move) -> bool {
        let legal_moves = self.get_legal_moves(self.white_turn);
        if legal_moves.contains(mv) {
            self.play_move_ob(mv);
            return true;
        }
        false
    }

    fn get_capture_moves(&self) -> Vec<Move> {
        let mut moves = self.get_legal_moves(self.white_turn);
        moves.retain(|x| self.board.state[x.target as usize] != 0);
        moves
    }

    fn get_all_moves_for_color(&self, white: bool) -> Vec<Move> {
        let mut moves = vec![];

        for square in 0..64 {
            let piece = self.board.state[square];
            if piece == 0 {
                continue;
            }
            let piece = Piece::init_from_binary(piece);
            if piece.is_white != white {
                continue;
            }
            let possible_moves = piece.possible_moves(square as u8, &self.board.clone());
            for move_position in possible_moves {
                moves.push(Move {
                    source: square as u8,
                    target: move_position,
                    promotion: 0u8,
                })
            }
        }
        moves
    }

    fn get_pseudolegal_moves(&self, source_square: String) -> Vec<String> {
        let position_index = position_helper::letter_to_index(source_square);
        let piece_opt = self.board.state.get(position_index as usize);
        if piece_opt.is_none() || piece_opt.is_some_and(|x| *x == 0u8) {
            return vec![];
        }

        let piece = Piece::init_from_binary(*piece_opt.unwrap());
        piece
            .possible_moves(position_index, &self.board)
            .iter()
            .map(|x| position_helper::index_to_letter(*x))
            .collect()
    }

    fn undo_move(&mut self) {
        if self.previous_fen_positions.is_empty() {
            return;
        }
        let last_move = self.previous_fen_positions.pop().unwrap();
        self.game_done = false;
        self.set_from_fen(last_move);
    }

    fn restart(&mut self) {
        let mut board = Board::init();
        board.set_start_position();
        self.white_turn = true;
        self.previous_fen_positions = vec![];
        self.board = board;
        self.game_done = false;
        self.en_passant = "-".to_string();
        self.half_move_clock = 0i32;
        self.full_move_number = 1i32;
    }

    fn play_move_ob(&mut self, chess_move: &Move) -> bool {
        self.play_move(chess_move.source, chess_move.target)
    }

    fn play_move_from_string(&mut self, source_square: String, target_square: String) -> bool {
        let initial_position_byte = position_helper::letter_to_index(source_square);
        let final_position_byte = position_helper::letter_to_index(target_square);
        self.play_move(initial_position_byte, final_position_byte)
    }

    fn set_from_fen(&mut self, fen: String) {
        // Check the size of the fen

        // Reset the board
        self.board.state = [0u8; 64];

        // Split the fen
        let mut fen_split = fen.split(' ');
        let board_state = fen_split.next().unwrap();
        let turn = fen_split.next().unwrap();
        let castling_options = fen_split.next().unwrap();
        let en_passant = fen_split.next().unwrap();
        let half_move_clock = fen_split.next().unwrap();
        let full_move_number = fen_split.next().unwrap();

        // Set the board state
        let mut board_state_index = 0;
        for c in board_state.chars() {
            if c == '/' {
                continue;
            }
            if c.is_numeric() {
                let num = c.to_digit(10).unwrap();
                board_state_index += num;
            } else {
                // Set the piece
                let mut piece = PIECE_BIT;
                if c.is_uppercase() {
                    piece += WHITE_BIT;
                }
                match c {
                    'p' | 'P' => piece += PAWN_BIT,
                    'r' | 'R' => piece += ROOK,
                    'n' | 'N' => piece += KNIGHT,
                    'b' | 'B' => piece += BISHOP,
                    'q' | 'Q' => piece += QUEEN,
                    'k' | 'K' => piece += KING,
                    _ => panic!("This piece does not exist!"),
                }
                let index: usize = board_state_index.try_into().unwrap();
                self.board.state[index] = piece;
                board_state_index += 1;
            }
        }

        // Check if the index reaqched 64
        if board_state_index != 64 {
            panic!("The board state is not complete");
        }

        // Set the turn
        self.white_turn = turn == "w";

        // Set castling options for board
        for c in castling_options.chars() {
            match c {
                'K' => self.board.castling |= 8u8,
                'Q' => self.board.castling |= 4u8,
                'k' => self.board.castling |= 2u8,
                'q' => self.board.castling |= 1u8,
                _ => (),
            }
        }

        // Set the en passant
        self.en_passant = en_passant.to_string();
        if en_passant != "-" {
            self.board.en_passant = position_helper::letter_to_index(en_passant.to_string());
        } else {
            self.board.en_passant = 0;
        }

        // Set the half move clock
        self.half_move_clock = half_move_clock.parse::<i32>().unwrap();

        // Set the full move number
        self.full_move_number = full_move_number.parse::<i32>().unwrap();
    }

    fn get_fen(&self) -> String {
        let mut fen_string = self.get_fen_simple();

        // Append the turn
        if self.white_turn {
            fen_string.push_str(" w ");
        } else {
            fen_string.push_str(" b ");
        }

        // Append the castling options
        fen_string.push_str(&self.board.get_castling_fen());

        fen_string.push(' ');

        // Append the en passant
        if self.board.en_passant == 0 {
            fen_string.push_str("-");
        } else {
            let en_passant = position_helper::index_to_letter(self.board.en_passant);
            fen_string.push_str(&en_passant);
        }
        fen_string.push(' ');

        // Append the half move clock
        fen_string.push_str(&self.half_move_clock.to_string());
        fen_string.push(' ');

        // Append the full move number
        fen_string.push_str(&self.full_move_number.to_string());

        fen_string
    }

    fn get_fen_simple(&self) -> String {
        let mut fen_string = "".to_string();
        let mut empty_count = 0;

        // Iterate through the board
        for i in 0..64 {
            let piece = self.board.state[i];
            if piece != 0 {
                if empty_count != 0 {
                    fen_string.push_str(&empty_count.to_string());
                }
                empty_count = 0;
                fen_string.push_str(&Piece::init_from_binary(piece).fen_repr());
            } else {
                empty_count += 1;
            }

            // Add number of empty slots by end of rank
            if (i + 1) % 8 == 0 && empty_count != 0 {
                fen_string.push_str(&empty_count.to_string());
            }

            // Add '/' at end of rank
            if (i + 1) % 8 == 0 && i != 63 {
                fen_string.push('/');
                empty_count = 0;
            }
        }

        fen_string
    }

    fn play_move(&mut self, source_idx: u8, target_idx: u8) -> bool {
        if self.game_done {
            let winning_side: String = if self.white_turn {
                "Black".to_string()
            } else {
                "White".to_string()
            };

            return false;
        }

        // Get the piece at the source index
        let piece_bits = self.board.state.get(source_idx as usize).unwrap_or(&0u8);

        // track changes
        let mut pawn_taken = false;

        let king_moved = false;
        let mut rook_moved = false;

        if piece_bits == &0u8 {
            return false;
        }

        let piece = Piece::init_from_binary(*piece_bits);

        // Check if turn is correct
        if piece.is_white != self.white_turn {
            return false;
        }

        let possible_moves = piece.possible_moves(source_idx, &self.board);
        // TODO: ensure these moves are actually legal, not just "pseudo legal"

        // Early return if the move is not possible
        if !possible_moves.contains(&target_idx) {
            return false;
        }

        // Move must be possible - continue
        // Update the previous positions vector
        let previous_fen = self.get_fen();

        // Take piece
        let taken_piece = self.board.state.get(target_idx as usize);
        let t_piece = taken_piece.unwrap_or(&0u8);
        if *t_piece != 0 {
            //TODO: store the piece taken and give rewards
            // TODO: This can be sped up by using the binary representation of the piece
            let taken_p = Piece::init_from_binary(*t_piece);
            if taken_p.class == PieceType::King {
                self.game_done = true;
            }
            if taken_p.class == PieceType::Pawn {
                pawn_taken = true;
            }
        }

        // Handle castling
        if piece.class == PieceType::King {
            let difference = target_idx as i32 - source_idx as i32;
            if difference.abs() == 2 {
                let king_side = difference > 0;
                if king_side {
                    let rook_pos = if piece.is_white { 63 } else { 7 };
                    let rook = Piece::init_from_binary(self.board.state[rook_pos as usize]);
                    self.update_board_object(&rook, rook_pos, rook_pos - 2, false);
                } else {
                    let rook_pos = if piece.is_white { 56 } else { 0 };
                    let rook = Piece::init_from_binary(self.board.state[rook_pos as usize]);
                    self.update_board_object(&rook, rook_pos, rook_pos + 3, false);
                }
            }
            // set castling options
            self.set_castling_options(true, true, true);
        }

        // Set en passant flag
        let en_passant_set: bool = self.set_en_passant_flag(&piece, source_idx, target_idx);

        // Update castling options if rook is moved
        if piece.class == PieceType::Rook {
            let is_kingside = position_helper::get_col(source_idx) == 7;
            rook_moved = true;
            self.set_castling_options(is_kingside, king_moved, rook_moved);
        }

        // Manage en passant taking
        self.en_passant_taking(&piece, target_idx);

        // update the board
        self.update_board_object(&piece, source_idx, target_idx, en_passant_set);
        self.previous_fen_positions.push(previous_fen);

        self.white_turn = !self.white_turn;

        //update the half move clock
        if piece.class == PieceType::Pawn || pawn_taken {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }

        // update full move count
        // TODO: ensure this generates the correct full move count
        if self.white_turn {
            self.full_move_number += 1;
        }

        true
    }
}

impl Game {
    fn set_en_passant_flag(&mut self, piece: &Piece, source_idx: u8, target_idx: u8) -> bool {
        let mut en_passant_set = false;
        if piece.class == PieceType::Pawn {
            let row_difference = position_helper::get_row(source_idx) as i32
                - position_helper::get_row(target_idx) as i32;
            if row_difference == 2 || row_difference == -2 {
                en_passant_set = true;
                if piece.is_white {
                    self.en_passant = position_helper::index_to_letter(target_idx + ROW);
                    self.board.en_passant = target_idx + ROW;
                } else {
                    self.en_passant = position_helper::index_to_letter(target_idx - ROW);
                    self.board.en_passant = target_idx - ROW;
                }
            }
        }
        en_passant_set
    }

    fn set_castling_options(&mut self, is_kingside: bool, king_moved: bool, rook_moved: bool) {
        if self.white_turn {
            if king_moved {
                self.board.castling &= 0b1111_0011;
            }
            if rook_moved & is_kingside {
                self.board.castling &= 0b1111_0111;
            } else if rook_moved & !is_kingside {
                self.board.castling &= 0b1111_1011;
            }
        } else {
            if king_moved {
                self.board.castling &= 0b1111_1100;
            }
            if rook_moved && is_kingside {
                self.board.castling &= 0b1111_1101;
            } else if rook_moved & !is_kingside {
                self.board.castling &= 0b1111_1110;
            }
        }
    }

    fn en_passant_taking(&mut self, piece: &Piece, target_idx: u8) {
        if piece.class == PieceType::Pawn
            && self.board.en_passant != 0
            && target_idx == self.board.en_passant
        {
            if piece.is_white {
                let pawn_taken_pos = self.board.en_passant + ROW;
                self.board.state[pawn_taken_pos as usize] = 0;
            } else {
                let pawn_taken_pos = self.board.en_passant - ROW;
                self.board.state[pawn_taken_pos as usize] = 0;
            }
        }
    }

    fn update_board_object(&mut self, piece: &Piece, source: u8, target: u8, en_passant_set: bool) {
        if !en_passant_set {
            self.board.en_passant = 0;
            self.en_passant = "-".to_string();
        }
        self.board.state[target as usize] = piece.binary;
        self.board.state[source as usize] = 0;
    }
}

pub mod position_helper {
    use crate::{Board, WHITE_BIT};

    pub fn index_to_letter(index: u8) -> String {
        let row_selector: u8 = 0b00111000;
        let col_selector: u8 = 0b00000111;

        let row = (row_selector & index) >> 3;
        let col = col_selector & index;

        let mut return_string = String::from("");

        let letter_char = (b'a' + col) as char;
        let num_char = (b'8' - row) as char;

        return_string.push(letter_char);
        return_string.push(num_char);
        return_string
    }

    pub fn letter_to_index(letters: String) -> u8 {
        let mut letters_copy = letters;
        let num_char = letters_copy.pop().unwrap();
        let letter_char = letters_copy.pop().unwrap();
        let row = 7 - (num_char as u8 - b'1');
        let col = letter_char as u8 - b'a';
        (row << 3) | col
    }

    pub fn get_row(byte: u8) -> u8 {
        let row_selector: u8 = 0b00111000;
        (row_selector & byte) >> 3
    }

    pub fn get_col(byte: u8) -> u8 {
        let col_selector: u8 = 0b00000111;
        col_selector & byte
    }

    fn validate_position(position: u8) -> bool {
        if position >= 64 {
            return false;
        }

        true
    }

    pub fn is_position_valid(
        destination_position: u8,
        board: &Board,
        is_piece_white: bool,
    ) -> bool {
        /*
        Checks whether position is within bounds and whether there is a same-coloured piece in the position
        */
        if !self::validate_position(destination_position) {
            return false;
        }

        let piece_opt = board.state.get(destination_position as usize);
        if piece_opt.is_some_and(|x| *x == 0u8) {
            return true;
        }

        let piece_byte = piece_opt.unwrap();
        let is_white = (piece_byte & WHITE_BIT) == WHITE_BIT;

        if is_white == is_piece_white {
            return false;
        }

        true
    }
}

pub mod engine {
    use crate::position_helper;
    use crate::psqt;
    use crate::Board;
    use crate::ChessGame;
    use crate::Game;
    use crate::Move;
    use crate::{BasicPiece, Piece, PieceType};

    pub struct Engine {
        pub game: Game,
    }

    impl Engine {
        pub fn init() -> Engine {
            Engine { game: Game::init() }
        }

        pub fn init_from_game(game: Game) -> Engine {
            Engine { game }
        }

        pub fn evaluate(board: &Board) -> i32 {
            let mut score = 0;

            // TODO: check for middle game and end game

            // Material
            for i in 0..64 {
                let piece = board.state[i];
                if piece == 0 {
                    continue;
                }
                let piece: Piece = Piece::init_from_binary(piece);
                let position_value = {
                    if piece.is_white {
                        match piece.class {
                            PieceType::King => 10000 + psqt::KING[i as usize],
                            PieceType::Queen => psqt::QUEEN[i as usize],
                            PieceType::Rook => psqt::ROOK[i as usize],
                            PieceType::Bishop => psqt::BISHOP[i as usize],
                            PieceType::Knight => psqt::KNIGHT[i as usize],
                            PieceType::Pawn => psqt::PAWN[i as usize],
                        }
                    } else {
                        match piece.class {
                            PieceType::King => 10000 + psqt::KING[psqt::FLIP[i as usize]],
                            PieceType::Queen => psqt::QUEEN[psqt::FLIP[i as usize]],
                            PieceType::Rook => psqt::ROOK[psqt::FLIP[i as usize]],
                            PieceType::Bishop => psqt::BISHOP[psqt::FLIP[i as usize]],
                            PieceType::Knight => psqt::KNIGHT[psqt::FLIP[i as usize]],
                            PieceType::Pawn => psqt::PAWN[psqt::FLIP[i as usize]],
                        }
                    }
                };
                if piece.is_white {
                    score += position_value;
                } else {
                    score -= position_value;
                }
            }
            score
        }

        pub fn get_best_move(&mut self, mut depth: u8) -> Move {
            let mut best_move = Move {
                source: 0,
                target: 0,
                promotion: 0,
            };

            let mut full_depth = depth * 2; // black and white move per depth
            let mut best_score = -100000;

            if self.game.white_turn {
                full_depth -= 1;
            }

            let moves = self.game.get_legal_moves(self.game.white_turn);
            for mv in moves {
                // make the move
                let success = self.game.play_move_ob(&mv);
                if !success {
                    continue;
                }
                let score = -self.alpha_beta(full_depth, best_score, -best_score);

                // undo the move
                self.game.undo_move();

                // update the best move
                if score > best_score {
                    best_score = score;
                    best_move = mv;
                }
            }
            let source = position_helper::index_to_letter(best_move.source);
            let target = position_helper::index_to_letter(best_move.target);
            println!("Best move: {}{} - score: {}", source, target, best_score);
            best_move
        }

        pub fn alpha_beta(&mut self, depth: u8, mut alpha: i32, beta: i32) -> i32 {
            if depth == 0 {
                return Engine::evaluate(&self.game.board);
            }
            let mut best_score = -100000;
            let moves = self.game.get_legal_moves(self.game.white_turn);
            for mv in moves {
                let success = self.game.play_move_ob(&mv);
                if !success {
                    continue;
                }
                let score = -self.alpha_beta(depth - 1, -beta, -alpha);
                self.game.undo_move();
                if score > best_score {
                    best_score = score;
                }
                if score > alpha {
                    alpha = score;
                }
                if alpha >= beta {
                    break;
                }
            }
            best_score
        }
    }
}

#[rustfmt::skip]
pub mod psqt {
    pub const PAWN: [i32; 64] = [
        100, 100, 100, 100, 100, 100, 100, 100,
        160, 160, 160, 160, 170, 160, 160, 160,
        140, 140, 140, 150, 160, 140, 140, 140,
        120, 120, 120, 140, 150, 120, 120, 120,
        105, 105, 115, 130, 140, 110, 105, 105,
        105, 105, 110, 120, 130, 105, 105, 105,
        105, 105, 105,  70,  70, 105, 105, 105,
        100, 100, 100, 100, 100, 100, 100, 100
    ];

    pub const KNIGHT: [i32; 64] = [
        290, 300, 300, 300, 300, 300, 300, 290,
        300, 305, 305, 305, 305, 305, 305, 300,
        300, 305, 325, 325, 325, 325, 305, 300,
        300, 305, 325, 325, 325, 325, 305, 300,
        300, 305, 325, 325, 325, 325, 305, 300,
        300, 305, 320, 325, 325, 325, 305, 300,
        300, 305, 305, 305, 305, 305, 305, 300,
        290, 310, 300, 300, 300, 300, 310, 290
    ];

    pub const BISHOP: [i32; 64] = [
        300, 320, 320, 320, 320, 320, 320, 300,
        305, 320, 320, 320, 320, 320, 320, 305,
        310, 320, 320, 325, 325, 320, 320, 310,
        310, 330, 330, 350, 350, 330, 330, 310,
        325, 325, 330, 345, 345, 330, 325, 325,
        325, 325, 325, 330, 330, 325, 325, 325,
        310, 325, 325, 330, 330, 325, 325, 310,
        300, 310, 310, 310, 310, 310, 310, 300
    ];

    pub const ROOK: [i32; 64] = [
        500, 500, 500, 500, 500, 500, 500, 500,
        515, 515, 515, 520, 520, 515, 515, 515,
        500, 500, 500, 500, 500, 500, 500, 500,
        500, 500, 500, 500, 500, 500, 500, 500,
        500, 500, 500, 500, 500, 500, 500, 500,
        500, 500, 500, 500, 500, 500, 500, 500,
        500, 500, 500, 500, 500, 500, 500, 500,
        500, 500, 500, 510, 510, 510, 500, 500
    ];

    pub const QUEEN: [i32; 64] = [
        870, 880, 890, 890, 890, 890, 880, 870,
        880, 890, 895, 895, 895, 895, 890, 880,
        890, 895, 910, 910, 910, 910, 895, 890,
        890, 895, 910, 920, 920, 910, 895, 890,
        890, 895, 910, 920, 920, 910, 895, 890,
        890, 895, 895, 895, 895, 895, 895, 890,
        880, 890, 895, 895, 895, 895, 890, 880,
        870, 880, 890, 890, 890, 890, 880, 870 
    ];

    pub const KING: [i32; 64] = [
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
         20, 20,  0,  0,  0,  0, 20, 20,
         20, 30, 10,  0,  0, 10, 30, 20
    ];

    pub const KING_LATE: [i32; 64] = [
        -50,-40,-30,-20,-20,-30,-40,-50,
        -30,-20,-10,  0,  0,-10,-20,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-30,  0,  0,  0,  0,-30,-30,
        -50,-30,-30,-30,-30,-30,-30,-50
    ];

    pub const FLIP: [usize; 64] = [
        63, 62, 61, 60, 59, 58, 57, 56,
        55, 54, 53, 52, 51, 50, 49, 48,
        47, 46, 45, 44, 43, 42, 41, 40,
        39, 38, 37, 36, 35, 34, 33, 32,
        31, 30, 29, 28, 27, 26, 25, 24,
        23, 22, 21, 20, 19, 18, 17, 16,
        15, 14, 13, 12, 11, 10,  9,  8,
        7,  6,  5,  4,  3,  2,  1,  0
    ];
}
