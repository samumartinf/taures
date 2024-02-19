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

use crate::piece::{BasicPiece, Piece};

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
