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

use crate::board::Board;
use crate::position_helper;

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
