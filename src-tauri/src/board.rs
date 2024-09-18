use crate::{BISHOP, KING, KNIGHT, PAWN_BIT, PIECE_BIT, QUEEN, ROOK, WHITE_BIT};

use crate::piece::{BasicPiece, Piece};

#[derive(Debug, Clone, Hash)]
/// Represents a chess board.
pub struct Board {
    /// The state of the chess board represented as an array of 64 bytes.
    /// Each index corresponds to a square on the board, and the value represents the piece on that square.
    pub state: [u8; 64],

    /// The bitboard representation of the chess board.
    /// The first 6 elements (0-5) represent the white pieces, and the next 6 elements (6-11) represent the black pieces.
    pub bitboard: [u64; 12],

    /// The hash value of the current board position.
    pub hash_value: u64,

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

/// Represents a chess board.
///
/// The `Board` struct provides methods for displaying the board, getting the castling FEN string,
/// initializing the board, getting the king position, and setting the start position.
impl Board {
    /// Displays the chess board.
    ///
    /// This method prints the current state of the chess board to the console.
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

    /// Gets the castling FEN string.
    ///
    /// This method returns the castling FEN string representing the castling rights of the board.
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

    /// Initializes a new chess board.
    ///
    /// This method creates a new instance of the `Board` struct with the initial state of the chess board with no pieces set.
    pub fn init() -> Self {
        let state = [0u8; 64];
        let bitboard = [0u64; 12];
        let hash = 0u64;
        let en_passant = 0u8;
        let castling = 8u8 + 4u8 + 2u8 + 1u8;
        Self {
            state,
            bitboard,
            hash_value: hash,
            en_passant,
            castling,
        }
    }

    /// Gets the position of the king.
    ///
    /// This method returns the position of the king on the board for the specified color.
    ///
    /// # Arguments
    ///
    /// * `is_white` - A boolean indicating whether the king is white or black.
    ///
    /// # Returns
    ///
    /// The position of the king as a u8 value. If the king is not found, 65 is returned.
    //TODO: change this to return an Option
    pub fn get_king_position(&self, is_white: bool) -> u8 {
        let king_byte = if is_white {
            PIECE_BIT + WHITE_BIT + KING
        } else {
            PIECE_BIT + KING
        };

        for i in 0..64 {
            if self.state[i] == king_byte {
                return i as u8;
            }
        }
        65
    }

    /// Sets the start position of the chess board.
    ///
    /// This method sets the chess board to the standard starting position.
    pub fn set_start_position(&mut self) {
        // Reset the board
        self.state = [0u8; 64];
        self.bitboard = [0u64; 12];

        // Set the bitboard
        // Black pieces
        // Part 1: Pawns
        self.bitboard[6] = 0b0000000011111111000000000000000000000000000000000000000000000000;
        // Part 2: Rooks
        self.bitboard[7] = 0b1000000100000000000000000000000000000000000000000000000000000000;
        // Part 3: Knights
        self.bitboard[8] = 0b0100001000000000000000000000000000000000000000000000000000000000;
        // Part 4: Bishops
        self.bitboard[9] = 0b0010010000000000000000000000000000000000000000000000000000000000;
        // Part 5: Queen
        self.bitboard[10] = 0b0000100000000000000000000000000000000000000000000000000000000000;
        // Part 6: King
        self.bitboard[11] = 0b0001000000000000000000000000000000000000000000000000000000000000;

        // White Pieces bitboard
        self.bitboard[0] = 0b0000000000000000000000000000000000000000000000001111111100000000;
        self.bitboard[1] = 0b0000000000000000000000000000000000000000000000000000000010000001;
        self.bitboard[2] = 0b0000000000000000000000000000000000000000000000000000000001000010;
        self.bitboard[3] = 0b0000000000000000000000000000000000000000000000000000000000100100;
        self.bitboard[4] = 0b0000000000000000000000000000000000000000000000000000000000001000;
        self.bitboard[5] = 0b0000000000000000000000000000000000000000000000000000000000010000;

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
