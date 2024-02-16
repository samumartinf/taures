use std::vec;

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
    fn play_move_ob(&mut self, chess_move: Move) -> bool;
    fn get_fen(&self) -> String;
    fn set_from_fen(&mut self, fen: String);
    fn get_fen_simple(&self) -> String;
    fn restart(&mut self);
    fn undo_move(&mut self);
    fn get_pseudolegal_moves(&self, position: String) -> Vec<String>;
    fn get_all_moves(&self) -> Vec<Move>;
    fn get_all_moves_for_color(&self, white: bool) -> Vec<Move>;
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

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub source: u8,    // source position byte
    pub target: u8,    // target position byte
    pub promotion: u8, // piece to promote to
}

impl ChessGame for Game {
    fn get_all_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        for square in 0..64 {
            let piece = self.board.state[square];
            if piece == 0 {
                continue;
            }
            let piece = Piece::init_from_binary(piece);
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

    fn play_move_ob(&mut self, chess_move: Move) -> bool {
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
        fen_string.push_str(&self.en_passant);
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

            println!("The Game is done {winning_side} won");
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
            println!("It is not your turn!");
            return false;
        }

        let possible_moves = piece.possible_moves(source_idx, &self.board);
        // TODO: ensure these moves are actually legal, not just "pseudo legal"

        // Early return if the move is not possible
        if !possible_moves.contains(&target_idx) {
            println!("This move is not valid");
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
                println!("GG wp");
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
    fn pawn_moves(self, source: u8, board: &Board) -> Vec<u8> {
        let mut possible_positions = Vec::new();

        // White pawns move in the negative direction
        let multiplier: i16 = if self.is_white { -1 } else { 1 };
        let one_row = (source as i16) + multiplier * (ROW as i16);
        let two_rows = (source as i16) + multiplier * (ROW as i16) * 2;

        let move_double_forward = if self.is_white { 6 } else { 1 };

        if board
            .state
            .get((one_row) as usize)
            .is_some_and(|x| *x == 0u8)
        {
            possible_positions.push(one_row);
        }

        if move_double_forward == position_helper::get_row(source)
            && board
                .state
                .get((two_rows) as usize)
                .is_some_and(|x| *x == 0u8)
            && board
                .state
                .get((one_row) as usize)
                .is_some_and(|x| *x == 0u8)
        {
            possible_positions.push(two_rows);
        }

        //Handle taking pieces
        let diagonal_right = (source as i16) + multiplier * (ROW as i16) + (COL as i16);
        let diagonal_left = (source as i16) + multiplier * (ROW as i16) - (COL as i16);

        //check the position to avoid taking on the other side
        let col = position_helper::get_col(source);

        if col < 7
            && (board
                .state
                .get(diagonal_right as usize)
                .is_some_and(|x| *x != 0u8)
                || board.en_passant == diagonal_right as u8)
        {
            possible_positions.push(diagonal_right);
        }

        if col > 0
            && (board
                .state
                .get(diagonal_left as usize)
                .is_some_and(|x| *x != 0u8)
                || board.en_passant == diagonal_left as u8)
        {
            possible_positions.push(diagonal_left);
        }

        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos as u8, board, self.is_white) {
                final_positions.push(pos as u8);
            }
        }

        final_positions
    }

    fn king_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let offsets = [-9, -8, -7, -1, 1, 7, 8, 9];
        let mut possible_positions = Vec::<u8>::new();
        let row = position_helper::get_row(position) as i16;
        let col = position_helper::get_col(position) as i16;
        for offset in offsets.iter() {
            let new_position = position as i16 + offset;
            if (position_helper::get_row(new_position as u8) as i16 - row).abs() > 1
                || (position_helper::get_col(new_position as u8) as i16 - col).abs() > 1
            {
                continue;
            }
            if position_helper::is_position_valid(new_position as u8, board, self.is_white) {
                possible_positions.push(new_position as u8);
            }
        }

        // Handle castling
        possible_positions.append(&mut self.castling_moves(position, board));

        possible_positions
    }

    fn castling_moves(self, source: u8, board: &Board) -> Vec<u8> {
        let mut possible_positions = Vec::<u8>::new();
        let mut king_side = false;
        let mut queen_side = false;

        if self.is_white {
            if board.castling & 8u8 == 8u8 {
                king_side = true;
            }
            if board.castling & 4u8 == 4u8 {
                queen_side = true;
            }
        } else {
            if board.castling & 2u8 == 2u8 {
                king_side = true;
            }
            if board.castling & 1u8 == 1u8 {
                queen_side = true;
            }
        }

        if king_side {
            let mut blocked = false;
            for i in 1..2 {
                let position_to_check = source + i;
                blocked = board.state[position_to_check as usize] != 0u8;
                if blocked {
                    break;
                }
            }
            let piece_at_rook = board.state[(source + 3) as usize];
            let rook = Piece::init_from_binary(piece_at_rook);
            if !blocked && rook.class == PieceType::Rook {
                possible_positions.push(source + 2);
            }
        }

        if queen_side {
            let mut blocked = false;
            for i in 1..3 {
                let position_to_check = source - i;
                blocked = board.state[position_to_check as usize] != 0u8;
                if blocked {
                    break;
                }
            }
            let piece_at_rook = board.state[(source - 4) as usize];
            let rook = Piece::init_from_binary(piece_at_rook);
            if !blocked && rook.class == PieceType::Rook {
                possible_positions.push(source - 2);
            }
        }

        possible_positions
    }

    fn rook_moves(self, source: u8, board: &Board) -> Vec<u8> {
        let mut possible_positions = Vec::<u8>::new();
        let row = position_helper::get_row(source);
        let col = position_helper::get_col(source);

        let mut blocked_right: bool = false;
        let mut blocked_up: bool = false;
        let mut blocked_down: bool = false;
        let mut blocked_left: bool = false;
        // move up, down, left, and right from the current position
        // check that there is no piece in the way
        for i in 1..8 {
            if col + i < 8 && !blocked_right {
                // check right boundary
                let position_to_check = source + i;
                let piece_retrieved = board.state.get(position_to_check as usize);

                // If a piece is found, we are now blocked from moving forward
                blocked_right = piece_retrieved.is_some_and(|x| *x != 0u8);
                possible_positions.push(source + i);
            }
            if i <= col && !blocked_left {
                // check left boundary
                let position_to_check = source - i;
                let piece_retrieved = board.state.get(position_to_check as usize);

                // If a piece is found, we are now blocked from moving forward
                blocked_left = piece_retrieved.is_some_and(|x| *x != 0u8);
                possible_positions.push(source - i);
            }
            if row + i < 8 && !blocked_down {
                // check lower boundary
                let position_to_check = source + ROW * i;
                let piece_retrieved = board.state.get(position_to_check as usize);

                // If a piece is found, we are now blocked from moving forward
                blocked_down = piece_retrieved.is_some_and(|x| *x != 0u8);
                possible_positions.push(source + ROW * i);
            }
            if i <= row && !blocked_up {
                // check upper boundary
                let position_to_check = source - ROW * i;
                let piece_retrieved = board.state.get(position_to_check as usize);

                blocked_up = piece_retrieved.is_some_and(|x| *x != 0u8);
                possible_positions.push(source - ROW * i);
            }
        }

        // Handle castling
        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos, board, self.is_white) {
                final_positions.push(pos);
            }
        }

        final_positions
    }

    fn queen_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let mut queen_positions = self.clone().rook_moves(position, board);
        let mut bishop_positions = self.bishop_moves(position, board);

        queen_positions.append(&mut bishop_positions);
        queen_positions.to_vec()
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
                        let piece_retrieved = board.state.get(position_to_check as usize);

                        blocked_down_right = piece_retrieved.is_some_and(|x| *x != 0u8);
                        moves.push(position + i + ROW * i);
                    }
                    if i <= row && !blocked_up_right {
                        let position_to_check = position + i - ROW * i;
                        let piece_retrieved = board.state.get(position_to_check as usize);

                        blocked_up_right = piece_retrieved.is_some_and(|x| *x != 0u8);
                        moves.push(position + i - ROW * i);
                    }
                }

                if i <= col {
                    if row + i < 8 && !blocked_down_left {
                        let position_to_check = position - i + ROW * i;
                        let piece_retrieved = board.state.get(position_to_check as usize);

                        blocked_down_left = piece_retrieved.is_some_and(|x| *x != 0u8);
                        moves.push(position - i + ROW * i);
                    }
                    if i <= row && !blocked_up_left {
                        let position_to_check = position - i - ROW * i;
                        let piece_retrieved = board.state.get(position_to_check as usize);

                        blocked_up_left = piece_retrieved.is_some_and(|x| *x != 0u8);
                        moves.push(position - i - ROW * i);
                    }
                }

                Some(moves)
            })
            .flatten()
            .filter(|&pos| position_helper::is_position_valid(pos, board, self.is_white))
            .collect()
    }

    fn knight_moves(self, position: u8, board: &Board) -> Vec<u8> {
        let offsets = [-17, -15, -10, -6, 6, 10, 15, 17];

        let mut possible_positions = Vec::<u8>::new();
        let row = position_helper::get_row(position) as i16;
        let col = position_helper::get_col(position) as i16;

        for offset in offsets.iter() {
            let new_position = position as i16 + offset;
            if (position_helper::get_row(new_position as u8) as i16 - row).abs() > 2
                || (position_helper::get_col(new_position as u8) as i16 - col).abs() > 2
            {
                continue;
            }
            if position_helper::is_position_valid(new_position as u8, board, self.is_white) {
                possible_positions.push(new_position as u8);
            }
        }
        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos, board, self.is_white) {
                final_positions.push(pos);
            }
        }

        final_positions
    }
}

impl BasicPiece for Piece {
    fn possible_moves(&self, position: u8, board: &Board) -> Vec<u8> {
        let possible_positions: Vec<u8> = match self.class {
            PieceType::Pawn => Piece::pawn_moves(self.clone(), position, board),
            PieceType::King => Piece::king_moves(self.clone(), position, board),
            PieceType::Bishop => Piece::bishop_moves(self.clone(), position, board),
            PieceType::Queen => Piece::queen_moves(self.clone(), position, board),
            PieceType::Rook => Piece::rook_moves(self.clone(), position, board),
            PieceType::Knight => Piece::knight_moves(self.clone(), position, board),
        };
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
        let color_string: String = if self.is_white {
            String::from("w")
        } else {
            String::from("b")
        };

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
        if !self.is_white {
            piece_string = piece_string.to_lowercase();
        }
        piece_string
    }
}

pub trait BasicPiece {
    fn init_from_binary(binary: u8) -> Self;
    fn text_repr(&self) -> String;
    fn possible_moves(&self, position: u8, board: &Board) -> Vec<u8>;
    fn fen_repr(&self) -> String;
}

#[derive(Debug, Clone)]
/// Represents a chess board.
pub struct Board {
    /// The state of the chess board represented as an array of 64 bytes.
    /// Each index corresponds to a square on the board, and the value represents the piece on that square.
    pub state: [u8; 64],

    /// The bitboard representation of the chess board.
    /// The first 6 elements (0-5) represent the white pieces, and the next 6 elements (6-11) represent the black pieces.
    pub bitboard: [u64; 12],

    /// The hash value of the current board position.
    pub hash: u64,

    /// The en passant square on the board.
    /// If no en passant square is available, it is set to 0.
    pub en_passant: u8,

    /// The castling rights of the players.
    /// The value is a bitmask where:
    /// - Bit 3 (8) represents white kingside castling (K)
    /// - Bit 2 (4) represents white queenside castling (Q)
    /// - Bit 1 (2) represents black kingside castling (k)
    /// - Bit 0 (1) represents black queenside castling (q)
    pub castling: u8,
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

    pub fn get_castling_fen(&self) -> String {
        let mut castling_fen = String::from("");
        if self.castling & 8u8 == 8u8 {
            castling_fen.push('K');
        }
        if self.castling & 4u8 == 4u8 {
            castling_fen.push('Q');
        }
        if self.castling & 2u8 == 2u8 {
            castling_fen.push('k');
        }
        if self.castling & 1u8 == 1u8 {
            castling_fen.push('q');
        }
        if castling_fen.is_empty() {
            castling_fen.push('-');
        }
        castling_fen
    }

    pub fn init() -> Self {
        let state = [0u8; 64];
        let bitboard = [0u64; 12];
        let hash = 0u64;
        let en_passant = 0u8;
        let castling = 8u8 + 4u8 + 2u8 + 1u8;
        Self {
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
        let mut fen_split = initial_fen.split(' ');
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

pub mod cherris_engine {
    use crate::Game;

    pub struct Engine {
        game: Game,
    }
}
