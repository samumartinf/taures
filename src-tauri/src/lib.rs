use std::{collections::HashMap, vec};

const PIECE_BIT: u8 = 128u8;
const WHITE_BIT: u8 = 64u8;
const PAWN_BIT: u8 = 8u8;
const CHECK_PIECE: u8 = 0b00001111;
const KING: u8 = 0u8;
const QUEEN: u8 = 1u8;
const BISHOP: u8 = 2u8;
const KNIGHT: u8 = 4u8;
const ROOK: u8 = 6u8;
const ROW: u8 = 16u8;
const COL: u8 = 1u8;

#[derive(Debug, Clone)]
pub struct Game {
    white_turn: bool,
    previous_fen_positions: Vec<String>,
    pub board: Board,
    game_done: bool,
    castling_options: Vec<char>,
    pub en_passant: String,
    half_move_clock: i32,
    full_move_number: i32,
}

pub trait ChessGame {
    fn play_move_from_string(&mut self, initial_position: String, final_position: String) -> bool;
    fn play_move(&mut self, initial_position: u8, final_position: u8) -> bool;
    fn get_fen(&self) -> String;
    fn set_from_fen(&mut self, fen: String);
    fn get_fen_simple(&self) -> String;
    fn restart(&mut self);
    fn undo_move(&mut self);
    fn get_pseudolegal_moves(&self, position: String) -> Vec<String>;
    fn get_all_moves(&self) -> Vec<Move>;
}

pub trait ChessDebugInfo {
    fn get_piece_at_square(&self, square: String) -> String;
}

impl ChessDebugInfo for Game {
    fn get_piece_at_square(&self, square: String) -> String {
        let position_byte = position_helper::letter_to_position_byte(square);
        let piece_opt = self.board.pieces.get(&position_byte);
        if piece_opt.is_none() {
            return String::from("None");
        }
        let piece = Piece::init_from_binary(*piece_opt.unwrap());
        return piece.fen_repr();
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
            castling_options: vec!['K', 'Q', 'k', 'q'],
            en_passant: "-".to_string(),
            half_move_clock: 0i32,
            full_move_number: 1i32,
        }
    }

    pub fn show(&self) {
        self.board.show();
    }
}

pub struct Move {
    source: u8,    // source position byte
    target: u8,    // target position byte
    promotion: u8, // piece to promote to
}

impl ChessGame for Game {
    fn get_all_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        for (position, piece) in &self.board.pieces {
            let piece = Piece::init_from_binary(*piece);
            let possible_moves = piece.possible_moves(*position, &self.board.clone());
            for move_position in possible_moves {
                moves.push(Move {
                    source: *position,
                    target: move_position,
                    promotion: 0u8,
                })
            }
        }

        return moves;
    }
    fn get_pseudolegal_moves(&self, position: String) -> Vec<String> {
        let position_byte = position_helper::letter_to_position_byte(position);
        let piece_opt = self.board.pieces.get(&position_byte);
        if piece_opt.is_none() {
            return vec![];
        }

        let piece = Piece::init_from_binary(*piece_opt.unwrap());
        return piece
            .possible_moves(position_byte, &self.board)
            .iter()
            .map(|x| position_helper::position_byte_to_letter(*x))
            .collect();
    }

    fn undo_move(&mut self) {
        if self.previous_fen_positions.is_empty() {
            return;
        }
        let last_move = self.previous_fen_positions.pop().unwrap();
        self.set_from_fen(last_move);
    }

    fn restart(&mut self) {
        let mut board = Board::init();
        board.set_start_position();
        self.white_turn = true;
        self.previous_fen_positions = vec![];
        self.board = board;
        self.game_done = false;
        self.castling_options = vec!['K', 'Q', 'k', 'q'];
        self.en_passant = "-".to_string();
        self.half_move_clock = 0i32;
        self.full_move_number = 1i32;
    }

    fn play_move_from_string(&mut self, initial_position: String, final_position: String) -> bool {
        let initial_position_byte = position_helper::letter_to_position_byte(initial_position);
        let final_position_byte = position_helper::letter_to_position_byte(final_position);
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

        self.board.update_hashmap();

        // Set the turn
        self.white_turn = turn == "w";

        // Set the castling options
        self.castling_options = vec![];
        for c in castling_options.chars() {
            self.castling_options.push(c);
        }

        // Set the en passant
        self.en_passant = en_passant.to_string();

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
        for option in &self.castling_options {
            fen_string.push_str(&option.to_string());
        }
        fen_string.push(' ');

        // Append the en passant
        fen_string.push_str(&self.en_passant);
        fen_string.push(' ');

        // Append the half move clock
        fen_string.push_str(&self.half_move_clock.to_string());
        fen_string.push(' ');

        // Append the full move number
        fen_string.push_str(&self.full_move_number.to_string());

        return fen_string;
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
            if (i + 1) % 8 == 0 && i != 63 {
                if empty_count != 0 {
                    fen_string.push_str(&empty_count.to_string());
                }
                fen_string.push('/');
                empty_count = 0;
            }
        }

        return fen_string;
    }

    fn play_move(&mut self, source: u8, target: u8) -> bool {
        let piece_opt = self.board.pieces.get(&source);
        let mut pawn_taken = false;
        let en_passant_set: bool;

        if piece_opt.is_none() {
            return false;
        }

        let piece = Piece::init_from_binary(*piece_opt.unwrap());

        // Check if turn is correct
        if piece.is_white != self.white_turn {
            println!("It is not your turn!");
            return false;
        }

        let possible_moves = piece.possible_moves(source, &self.board);
        // TODO: ensure these moves are actually legal, not just "pseudo legal"
        if possible_moves.contains(&target) {
            // Update the previous positions vector
            let previous_fen = self.get_fen();

            // Take piece
            let final_position_index = position_helper::position_byte_to_index(target);
            let taken_piece = self.board.state.get(final_position_index);
            let t_piece = taken_piece.unwrap_or(&0u8);
            if *t_piece != 0 {
                //TODO: store the piece taken and give rewards
                let taken_p = Piece::init_from_binary(*t_piece);
                if taken_p.class == PieceType::King {
                    self.game_done = true;
                    println!("GG wp");
                }
                if taken_p.class == PieceType::Pawn {
                    pawn_taken = true;
                }
            }

            // Set en passant flag
            en_passant_set = self.set_en_passant_flag(&piece, source, target);

            // Manage en passant taking
            self.en_passant_taking(&piece, target);

            // update the board
            self.update_board_object(&piece, source, target, en_passant_set);
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

            return true;
        } else {
            println!("This move is not valid");
        }
        return false;
    }
}

impl Game {
    fn set_en_passant_flag(&mut self, piece: &Piece, source: u8, target: u8) -> bool {
        let mut en_passant_set = false;
        if piece.class == PieceType::Pawn {
            let row_difference =
                position_helper::get_row(source) as i32 - position_helper::get_row(target) as i32;
            if row_difference == 2 || row_difference == -2 {
                en_passant_set = true;
                if piece.is_white {
                    self.en_passant = position_helper::position_byte_to_letter(target + 16);
                    self.board.en_passant = target + 16;
                } else {
                    self.en_passant = position_helper::position_byte_to_letter(target - 16);
                    self.board.en_passant = target - 16;
                }
            }
        }
        return en_passant_set;
    }

    fn en_passant_taking(&mut self, piece: &Piece, target: u8) {
        if piece.class == PieceType::Pawn
            && self.board.en_passant != 0
            && target == self.board.en_passant
        {
            if piece.is_white {
                let pawn_taken_pos = self.board.en_passant + 16;
                self.board.state
                    [position_helper::position_byte_to_index(pawn_taken_pos)] = 0;
            } else {
                let pawn_taken_pos = self.board.en_passant - 16;
                self.board.state
                    [position_helper::position_byte_to_index(pawn_taken_pos)] = 0;
            }
        }
    }

    fn update_board_object(&mut self, piece: &Piece, source: u8, target: u8, en_passant_set: bool) {
        if !en_passant_set {
            self.board.en_passant = 0;
            self.en_passant = "-".to_string();
        }
        self.board.state[position_helper::position_byte_to_index(target) as usize] = piece.binary;
        self.board.state[position_helper::position_byte_to_index(source)] = 0;

        self.board.update_hashmap();
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub binary: u8,
    pub is_white: bool,
    pub class: PieceType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    fn pawn_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let mut possible_positions = Vec::new();

        // White pawns move in the negative direction
        if self.is_white {
            if board.pieces.get(&(position - 16)).is_none() {
                possible_positions.push(position - 16);
            }

            if position_helper::get_row(position) == 6
                && board.pieces.get(&(position - 32)).is_none()
            {
                possible_positions.push(position - 32);
            }
        }
        // Black paws move in the positive direction
        else {
            if board.pieces.get(&(position + 16)).is_none() {
                possible_positions.push(position + 16);
            }
            if position_helper::get_row(position) == 1
                && board.pieces.get(&(position + 32)).is_none()
            {
                possible_positions.push(position + 32);
            }
        }
        //Handle taking pieces
        if self.is_white {
            let diagonal_right = position - ROW + COL;
            let diagonal_left = position - ROW - COL;
            if board.pieces.get(&diagonal_left.clone()).is_some()
                || board.en_passant == diagonal_left
            {
                possible_positions.push(diagonal_left.clone());
            }
            if board.pieces.get(&diagonal_right.clone()).is_some()
                || board.en_passant == diagonal_right
            {
                possible_positions.push(diagonal_right.clone());
            }
        }
        // Black paws move in the positive direction
        else {
            let diagonal_right = position + ROW + COL;
            let diagonal_left = position + ROW - COL;
            if board.pieces.get(&diagonal_left.clone()).is_some()
                || board.en_passant == diagonal_left
            {
                possible_positions.push(diagonal_left.clone());
            }
            if board.pieces.get(&diagonal_right.clone()).is_some()
                || board.en_passant == diagonal_right
            {
                possible_positions.push(diagonal_right.clone());
            }
        }

        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos, &board, self.is_white) {
                final_positions.push(pos);
            }
        }

        final_positions
    }

    fn king_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let mut possible_positions = Vec::<u8>::new();
        let row = position_helper::get_row(position);
        let col = position_helper::get_col(position);

        if col > 0 {
            if let Some(new_position) = position.checked_sub(COL) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_sub(ROW + COL) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_add(ROW - COL) {
                possible_positions.push(new_position);
            }
        }
        if col < 7 {
            if let Some(new_position) = position.checked_add(COL) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_sub(ROW - COL) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_add(ROW + COL) {
                possible_positions.push(new_position);
            }
        }
        if row > 0 {
            if let Some(new_position) = position.checked_sub(ROW) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_sub(ROW + COL) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_add(ROW - COL) {
                possible_positions.push(new_position);
            }
        }
        if row < 7 {
            if let Some(new_position) = position.checked_add(ROW) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_add(ROW - COL) {
                possible_positions.push(new_position);
            }
            if let Some(new_position) = position.checked_add(ROW + COL) {
                possible_positions.push(new_position);
            }
        }

        let mut final_positions = Vec::<u8>::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos, &board, self.is_white) {
                final_positions.push(pos);
            }
        }

        final_positions
    }

    fn rook_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let mut possible_positions = Vec::<u8>::new();
        let row = position_helper::get_row(position);
        let col = position_helper::get_col(position);

        let mut blocked_right: bool = false;
        let mut blocked_up: bool = false;
        let mut blocked_down: bool = false;
        let mut blocked_left: bool = false;
        // move up, down, left, and right from the current position
        // check that there is no piece in the way
        for i in 1..8 {
            if col + i < 8 && !blocked_right {
                // check right boundary
                let position_to_check = position + i;
                let piece_retrieved = board.pieces.get(&position_to_check);

                // If a piece is found, we are now blocked from moving forward
                blocked_right = piece_retrieved.is_some();
                possible_positions.push(position + i);
            }
            if i <= col && !blocked_left {
                // check left boundary
                let position_to_check = position - i;
                let piece_retrieved = board.pieces.get(&position_to_check);

                // If a piece is found, we are now blocked from moving forward
                blocked_left = piece_retrieved.is_some();
                possible_positions.push(position - i);
            }
            if row + i < 8 && !blocked_down {
                // check lower boundary
                let position_to_check = position + ROW * i;
                let piece_retrieved = board.pieces.get(&position_to_check);

                // If a piece is found, we are now blocked from moving forward
                blocked_down = piece_retrieved.is_some();
                possible_positions.push(position + ROW * i);
            }
            if i <= row && !blocked_up {
                // check upper boundary
                let position_to_check = position - ROW * i;
                let piece_retrieved = board.pieces.get(&position_to_check);

                blocked_up = piece_retrieved.is_some();
                possible_positions.push(position - ROW * i);
            }
        }

        // Handle castling

        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos, &board, self.is_white) {
                final_positions.push(pos);
            }
        }

        return final_positions;
    }

    fn queen_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let mut queen_positions = self.clone().rook_moves(position, &board);
        let mut bishop_positions = self.bishop_moves(position, &board);

        queen_positions.append(&mut bishop_positions);
        return queen_positions.to_vec();
    }

    fn bishop_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let row = position_helper::get_row(position);
        let col = position_helper::get_col(position);
        let mut blocked_up_left = false;
        let mut blocked_down_left = false;
        let mut blocked_up_right = false;
        let mut blocked_down_right = false;

        (1..8)
            .filter_map(|i| {
                let mut moves = Vec::new();

                if col + i < 8 {
                    if row + i < 8 && !blocked_down_right {
                        let position_to_check = position + i + ROW * i;
                        let piece_retrieved = board.pieces.get(&position_to_check);

                        blocked_down_right = piece_retrieved.is_some();
                        moves.push(position + i + ROW * i);
                    }
                    if i <= row && !blocked_up_right {
                        let position_to_check = position + i - ROW * i;
                        let piece_retrieved = board.pieces.get(&position_to_check);

                        blocked_up_right = piece_retrieved.is_some();
                        moves.push(position + i - ROW * i);
                    }
                }

                if i <= col {
                    if row + i < 8 && !blocked_down_left {
                        let position_to_check = position - i + ROW * i;
                        let piece_retrieved = board.pieces.get(&position_to_check);

                        blocked_down_left = piece_retrieved.is_some();
                        moves.push(position - i + ROW * i);
                    }
                    if i <= row && !blocked_up_left {
                        let position_to_check = position - i - ROW * i;
                        let piece_retrieved = board.pieces.get(&position_to_check);

                        blocked_up_left = piece_retrieved.is_some();
                        moves.push(position - i - ROW * i);
                    }
                }

                Some(moves)
            })
            .flatten()
            .filter(|&pos| position_helper::is_position_valid(pos, &board, self.is_white))
            .collect()
    }

    fn knight_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let possible_positions: Vec<u8> = vec![
            position
                .checked_add(COL)
                .and_then(|x| x.checked_add(2 * ROW)),
            position
                .checked_sub(COL)
                .and_then(|x| x.checked_add(2 * ROW)),
            position
                .checked_add(COL)
                .and_then(|x| x.checked_sub(2 * ROW)),
            position
                .checked_sub(COL)
                .and_then(|x| x.checked_sub(2 * ROW)),
            position
                .checked_add(ROW)
                .and_then(|x| x.checked_add(2 * COL)),
            position
                .checked_add(ROW)
                .and_then(|x| x.checked_sub(2 * COL)),
            position
                .checked_sub(ROW)
                .and_then(|x| x.checked_add(2 * COL)),
            position
                .checked_sub(ROW)
                .and_then(|x| x.checked_sub(2 * COL)),
        ]
        .into_iter()
        .flatten()
        .collect();

        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos, &board, self.is_white) {
                final_positions.push(pos);
            }
        }

        return final_positions;
    }
}

impl BasicPiece for Piece {
    fn possible_moves(&self, position: u8, board: &Board) -> Vec<u8> {
        let possible_positions:Vec<u8>;
        match self.class {
            PieceType::Pawn => {
                possible_positions = Piece::pawn_moves(self.clone(), position, &board)
            }
            PieceType::King => {
                possible_positions = Piece::king_moves(self.clone(), position, board)
            }
            PieceType::Bishop => {
                possible_positions = Piece::bishop_moves(self.clone(), position, board)
            }
            PieceType::Queen => {
                possible_positions = Piece::queen_moves(self.clone(), position, board)
            }
            PieceType::Rook => {
                possible_positions = Piece::rook_moves(self.clone(), position, board)
            }
            PieceType::Knight => {
                possible_positions = Piece::knight_moves(self.clone(), position, board)
            }
        }

        possible_positions
    }

    fn init_from_binary(binary: u8) -> Self {
        let is_white = (binary & WHITE_BIT) == WHITE_BIT;
        // The alive bit might mess this up
        let binary_piece = binary & CHECK_PIECE;

        let piece_type = match binary_piece {
            8u8..=16u8 => PieceType::Pawn,
            0u8 => PieceType::King,
            1u8 => PieceType::Queen,
            2u8 | 3u8 => PieceType::Bishop,
            4u8 | 5u8 => PieceType::Knight,
            6u8 | 7u8 => PieceType::Rook,
            _ => panic!("This piece does not exist!. The binary is {}", binary),
        };

        Self {
            binary,
            is_white,
            class: piece_type,
        }
    }

    fn text_repr(&self) -> String {
        let mut return_string = String::from("");
        let color_string :String;

        if self.is_white {
            color_string = String::from("w");
        } else {
            color_string = String::from("b");
        }

        let piece_string = match self.class {
            PieceType::Pawn => "P".to_string(),
            PieceType::King => "K".to_string(),
            PieceType::Queen => "Q".to_string(),
            PieceType::Bishop => "B".to_string(),
            PieceType::Knight => "N".to_string(),
            PieceType::Rook => "R".to_string(),
        };
        return_string.push_str(&color_string);
        return_string.push_str(&piece_string);
        return_string
    }

    fn fen_repr(&self) -> String {
        let mut piece_string = match self.class {
            PieceType::Pawn => "P",
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Bishop => "B",
            PieceType::Knight => "N",
            PieceType::Rook => "R",
        }
        .to_string();
        if self.is_white == false {
            piece_string = piece_string.to_lowercase();
        }
        return piece_string;
    }
}

pub trait BasicPiece {
    fn init_from_binary(binary: u8) -> Self;
    fn text_repr(&self) -> String;
    fn possible_moves(&self, position: u8, board: &Board) -> Vec<u8>;
    fn fen_repr(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: HashMap<u8, u8>, // HashMap<positionByte, pieceByte>
    pub state: [u8; 64],         // arr[index] = pieceByte
    pub bitboard: [u64; 12],     // 0-5 white, 6-11 black (unsured atm)
    pub hash: u64,
    pub en_passant: u8, // 0000 0000 or the position
    pub castling: u8,   // 8 = K, 4 = Q, 2 = k, 1 = q
}

impl Board {
    pub fn show(&self) {
        println!("  |----|----|----|----|----|----|----|----|");
        let mut row_count = 8;
        for row in 0..8 {
            print!("{} ", row_count);
            row_count -= 1;
            print!("|");

            for col in 0..8 {
                print!(" ");

                // Piece print
                if self.state[row * 8 + col] == 0u8 {
                    print!("  ");
                } else {
                    let piece = Piece::init_from_binary(self.state[row * 8 + col]);
                    print!("{}", piece.text_repr());
                }

                print!(" |");
            }
            println!();
            println!("  |----|----|----|----|----|----|----|----|");
        }
        println!("    a    b    c    d    e    f    g    h  ");
    }

    pub fn init() -> Self {
        let state = [0u8; 64];
        let pieces: HashMap<u8, u8> = HashMap::new();
        let bitboard = [0u64; 12];
        let hash = 0u64;
        let en_passant = 0u8;
        let castling = 8u8 + 4u8 + 2u8 + 1u8;
        Self {
            pieces,
            state,
            bitboard,
            hash,
            en_passant,
            castling,
        }
    }

    pub fn set_start_position(&mut self) {
        // Reset the board
        self.state = [0u8; 64];

        let initial_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        // Split the fen
        let mut fen_split = initial_fen.split(" ");
        let board_state = fen_split.next().unwrap();

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
                self.state[index] = piece;
                board_state_index += 1;
            }
        }
        self.update_hashmap();
    }

    pub fn update_hashmap(&mut self) {
        // TODO: optimise this to avoid memory allocation
        self.pieces = HashMap::new();
        for index in 0..self.state.len() {
            if self.state[index] != 0 {
                let pos_byte = position_helper::index_to_position_byte(index);
                self.pieces.insert(pos_byte, self.state[index]);
            }
        }
    }
}

pub mod position_helper {
    use crate::{Board, WHITE_BIT};

    pub fn position_byte_to_index(byte: u8) -> usize {
        let row_selector: u8 = 0b11110000;
        let col_selector: u8 = 0b00001111;

        let row = (row_selector & byte) >> 4;
        let col = col_selector & byte;

        (row * 8 + col) as usize
    }

    pub fn index_to_position_byte(index: usize) -> u8 {
        let col = index as u8 % 8;
        let mut row = index as u8 / 8u8;
        row <<= 4;
        row | col
    }

    pub fn position_byte_to_letter(byte: u8) -> String {
        let row_selector: u8 = 0b11110000;
        let col_selector: u8 = 0b00001111;

        let row = (row_selector & byte) >> 4;
        let col = col_selector & byte;

        let mut return_string = String::from("");

        let letter_char = (b'a' + col) as char;
        let num_char = (b'8' - row) as char;

        return_string.push(letter_char);
        return_string.push(num_char);
        return_string
    }

    pub fn letter_to_position_byte(letters: String) -> u8 {
        let mut letters_copy = letters;
        let num_char = letters_copy.pop().unwrap();
        let letter_char = letters_copy.pop().unwrap();
        let row = 7 - (num_char as u8 - b'1');
        let col = letter_char as u8 - b'a';
        (row << 4) | col
    }

    pub fn get_row(byte: u8) -> u8 {
        let row_selector: u8 = 0b11110000;
        (row_selector & byte) >> 4
    }

    pub fn get_col(byte: u8) -> u8 {
        let col_selector: u8 = 0b00001111;
        col_selector & byte
    }

    fn validate_position(position: u8) -> bool {
        let index_position = position_byte_to_index(position);
        if index_position >= 64 {
            return false;
        }
        if get_col(position) > 7 {
            return false;
        }
        if get_row(position) > 7 {
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
        if self::validate_position(destination_position) == false {
            return false;
        }

        let piece_opt = board.pieces.get(&destination_position);
        if piece_opt.is_none() {
            return true;
        }

        let piece_byte = piece_opt.unwrap();
        let is_white = (piece_byte & WHITE_BIT) == WHITE_BIT;

        if is_white == is_piece_white {
            return false;
        }

        return true;
    }
}

pub mod cherris_engine {
    use crate::Game;

    pub struct Engine {
        game: Game,
    }
}
