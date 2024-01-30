use std::{collections::HashMap, io, vec};

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

pub struct Game {
    white_turn: bool,
    moves_done: Vec<u32>,
    board: Board,
    game_done: bool,
}

pub trait ChessGame {
    fn play(&mut self);
    fn play_move(&mut self, initial_position: u8, final_position: u8) -> bool;
}

impl Game {
    pub fn init() -> Game {
        let mut board = Board::init();
        board.update_hashmap();
        Game {
            white_turn: true,
            moves_done: vec![],
            board,
            game_done: false,
        }
    }
}

impl ChessGame for Game {
    fn play(&mut self) {
        while !self.game_done {
            let mut i_position_string = String::new();
            let mut f_position_string = String::new();

            self.board.show();

            if self.white_turn {
                println!("WHITE TURN");
            } else {
                println!("BLACK TURN");
            }

            println!("Piece initial pos: ");
            io::stdin()
                .read_line(&mut i_position_string)
                .expect("Failed to read line");

            i_position_string = i_position_string.trim().to_string();
            let i_position = position_helper::letter_to_position_byte(i_position_string);

            println!("Move: ");
            io::stdin()
                .read_line(&mut f_position_string)
                .expect("Failed to read line");

            f_position_string = f_position_string.trim().to_string();
            let f_position = position_helper::letter_to_position_byte(f_position_string);
            let move_was_valid = self.play_move(i_position, f_position);

            //End of turn
            if move_was_valid {
                self.white_turn = !self.white_turn;
            }
        }
    }

    fn play_move(&mut self, initial_position: u8, final_position: u8) -> bool {
        let piece_opt = self.board.pieces.get(&initial_position);

        if let Some(piece_bits) = piece_opt {
            let piece = Piece::init_from_binary(*piece_bits);
            let possible_moves = piece.possible_moves(initial_position, &self.board);
            if possible_moves.contains(&final_position) {
                // Take piece
                let final_position_index = position_helper::position_byte_to_index(final_position);
                let taken_piece = self.board.state.get(final_position_index as usize);
                let t_piece = taken_piece.unwrap_or(&0u8);
                if *t_piece != 0 {
                    //TODO: store the piece taken and give rewards
                    if Piece::init_from_binary(*taken_piece.unwrap()).class == PieceType::King {
                        self.game_done = true;
                        println!("GG wp");
                    }
                }
                // update the board
                self.board.state[final_position_index as usize] = piece.binary;
                self.board.state[position_helper::position_byte_to_index(initial_position)] = 0;

                self.board.update_hashmap();
                return true;
            } else {
                println!("This move is not valid");
            }
        }
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    binary: u8,
    is_white: bool,
    class: PieceType,
}

#[derive(Debug, Clone, PartialEq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    fn pawn_moves(self, position: u8, board: Board) -> Vec<u8> {
        let mut possible_positions = Vec::new();

        // White pawns move in the negative direction
        if self.is_white {
            possible_positions.push(position - 16);

            if position_helper::get_row(position) == 6 {
                possible_positions.push(position - 32);
            }
        }
        // Black paws move in the positive direction
        else {
            possible_positions.push(position + 16);
            if position_helper::get_row(position) == 1 {
                possible_positions.push(position + 32);
            }
        }
        //Handle taking pieces
        if self.is_white {
            let diagonal_right = position - ROW + COL;
            let diagonal_left = position - ROW - COL;
            if board.pieces.get(&diagonal_left.clone()).is_some() {
                possible_positions.push(diagonal_left.clone());
            }
            if board.pieces.get(&diagonal_right.clone()).is_some() {
                possible_positions.push(diagonal_right.clone());
            }
        }

        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::validate_position(pos) {
                final_positions.push(pos);
            }
        }

        final_positions
    }

    fn king_moves(self, position: u8, _board: Board) -> Vec<u8> {
        let possible_positions = vec![
            position + COL,
            position - COL,
            position + ROW,
            position - ROW,
            position + ROW + COL,
            position + ROW - COL,
            position - ROW + COL,
            position - ROW - COL,
        ];

        let mut final_positions: Vec<u8> = Vec::new();
        for pos in possible_positions {
            if position_helper::validate_position(pos) {
                final_positions.push(pos);
            }
        }

        final_positions
    }

    fn rook_moves(self, position: u8, board: Board) -> Vec<u8> {
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

        let mut final_positions = Vec::new();
        for pos in possible_positions {
            if position_helper::is_position_valid(pos, &board, self.is_white) {
                final_positions.push(pos);
            }
        }

        return final_positions;
    }

    fn queen_moves(self, position: u8, board: Board) -> Vec<u8> {
        let row = position_helper::get_row(position);
        let col = position_helper::get_col(position);

        let mut queen_positions = self.clone().rook_moves(position, board.clone());
        let mut bishop_positions = self.bishop_moves(position, board);

        queen_positions.append(&mut bishop_positions);
        return queen_positions.to_vec();

    }

    fn bishop_moves(self, position: u8, board: Board) -> Vec<u8> {
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

                        blocked_down_left= piece_retrieved.is_some();
                        moves.push(position - i + ROW * i);
                    }
                    if i <= row && !blocked_up_left{
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

    fn knight_moves(self, position: u8, board: Board) -> Vec<u8> {
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
            if position_helper::validate_position(pos) {
                final_positions.push(pos);
            }
        }

        return final_positions;
    }
}

impl BasicPiece for Piece {
    fn is_move_valid(&self, position: u8, board: Board) -> bool {
        //TODO: implement this

        return true;
    }

    fn possible_moves(&self, position: u8, board: &Board) -> Vec<u8> {
        let mut possible_positions = Vec::new();
        match self.class {
            PieceType::Pawn => {
                possible_positions = Piece::pawn_moves(self.clone(), position, board.clone())
            }
            PieceType::King => {
                possible_positions = Piece::king_moves(self.clone(), position, board.clone())
            }
            PieceType::Bishop => {
                possible_positions = Piece::bishop_moves(self.clone(), position, board.clone())
            }
            PieceType::Queen => {
                possible_positions = Piece::queen_moves(self.clone(), position, board.clone())
            }
            PieceType::Rook => {
                possible_positions = Piece::rook_moves(self.clone(), position, board.clone())
            }
            PieceType::Knight => {
                possible_positions = Piece::knight_moves(self.clone(), position, board.clone())
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
        let mut color_string = String::from("");

        if self.is_white {
            color_string = String::from("w");
        } else {
            color_string = String::from("b");
        }

        let piece_string = match self.class {
            PieceType::Pawn => "p".to_string(),
            PieceType::King => "K".to_string(),
            PieceType::Queen => "Q".to_string(),
            PieceType::Bishop => "B".to_string(),
            PieceType::Knight => "k".to_string(),
            PieceType::Rook => "R".to_string(),
        };
        return_string.push_str(&color_string);
        return_string.push_str(&piece_string);
        return_string
    }
}

pub trait BasicPiece {
    fn is_move_valid(&self, position: u8, board: Board) -> bool;
    fn init_from_binary(binary: u8) -> Self;
    fn text_repr(&self) -> String;
    fn possible_moves(&self, position: u8, board: &Board) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: HashMap<u8, u8>, // HashMap<positionByte, pieceByte>
    pub state: [u8; 64],         // arr[index] = pieceByte
}

impl Board {
    fn show(&self) {
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
        let mut state = [0u8; 64];
        let pieces: HashMap<u8, u8> = HashMap::new();

        // black pawns
        let mut first_bpawn = PIECE_BIT + PAWN_BIT;
        for i in 0..8 {
            state[i + 8] = first_bpawn;
            first_bpawn += 1;
        }

        // white pawns
        let mut first_wpawn = PIECE_BIT + PAWN_BIT + WHITE_BIT;
        for i in 0..8 {
            state[i + 48] = first_wpawn;
            first_wpawn += 1;
        }

        // white large pieces
        state[56] = ROOK + PIECE_BIT + WHITE_BIT;
        state[1 + 56] = KNIGHT + PIECE_BIT + WHITE_BIT;
        state[2 + 56] = BISHOP + PIECE_BIT + WHITE_BIT;
        state[3 + 56] = QUEEN + PIECE_BIT + WHITE_BIT;
        state[4 + 56] = KING + PIECE_BIT + WHITE_BIT;
        state[5 + 56] = BISHOP + PIECE_BIT + WHITE_BIT + 1;
        state[6 + 56] = KNIGHT + PIECE_BIT + WHITE_BIT + 1;
        state[7 + 56] = ROOK + PIECE_BIT + WHITE_BIT + 1;

        // black large pieces
        state[0] = ROOK + PIECE_BIT;
        state[1] = KNIGHT + PIECE_BIT;
        state[2] = BISHOP + PIECE_BIT;
        state[3] = QUEEN + PIECE_BIT;
        state[4] = KING + PIECE_BIT;
        state[5] = BISHOP + PIECE_BIT + 1;
        state[6] = KNIGHT + PIECE_BIT + 1;
        state[7] = ROOK + PIECE_BIT + 1;

        // Populate hashmap -> done in the update_hashmap
        Self { pieces, state }
    }

    pub fn update_hashmap(&mut self) {
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

    pub fn validate_position(position: u8) -> bool {
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