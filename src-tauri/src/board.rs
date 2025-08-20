use crate::{BISHOP, KING, KNIGHT, PAWN_BIT, PIECE_BIT, QUEEN, ROOK, WHITE_BIT};
use crate::masks;
use crate::piece::{BasicPiece, Piece};

#[derive(Debug, Clone, Hash)]
/// Represents a chess board.
pub struct Board {
    /// The traditional array-based board representation.
    /// Each element represents a square with piece information encoded as bytes.
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

        // White Pieces bitboard - same order as above
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
        
        // Update bitboards to match the array state
        self.update_bitboards_from_array();
    }

    // Bitboard utility methods
    
    /// Updates all bitboards based on the current array state
    pub fn update_bitboards_from_array(&mut self) {
        // Clear all bitboards
        self.bitboard = [0u64; 12];
        
        // Iterate through the array and set corresponding bits in bitboards
        for square in 0..64 {
            let piece = self.state[square];
            if piece == 0 {
                continue;
            }
            
            let is_white = (piece & WHITE_BIT) != 0;
            let piece_type = piece & 0b00001111; // Get piece type bits
            
            let bitboard_index = self.get_bitboard_index(piece_type, is_white);
            self.bitboard[bitboard_index] |= 1u64 << square;
        }
    }
    
    /// Updates the array based on current bitboard state
    pub fn update_array_from_bitboards(&mut self) {
        // Clear the array
        self.state = [0u8; 64];
        
        // Iterate through all bitboards and set pieces in array
        for bitboard_index in 0..12 {
            let mut bb = self.bitboard[bitboard_index];
            let (piece_type, is_white) = self.get_piece_info_from_bitboard_index(bitboard_index);
            
            while bb != 0 {
                let square = self.pop_lsb(&mut bb);
                self.state[square] = self.encode_piece(piece_type, is_white);
            }
        }
    }
    
    /// Gets the bitboard index for a piece type and color
    fn get_bitboard_index(&self, piece_type: u8, is_white: bool) -> usize {
        let base_index = match piece_type {
            t if t == PAWN_BIT => 0,
            t if t == ROOK => 1, 
            t if t == KNIGHT => 2,
            t if t == BISHOP => 3,
            t if t == QUEEN => 4,
            t if t == KING => 5,
            _ => panic!("Invalid piece type: {}", piece_type),
        };
        
        if is_white {
            base_index
        } else {
            base_index + 6
        }
    }
    
    /// Gets piece type and color from bitboard index
    fn get_piece_info_from_bitboard_index(&self, index: usize) -> (u8, bool) {
        let is_white = index < 6;
        let piece_index = index % 6;
        
        let piece_type = match piece_index {
            0 => PAWN_BIT,
            1 => ROOK,
            2 => KNIGHT,
            3 => BISHOP,
            4 => QUEEN,
            5 => KING,
            _ => panic!("Invalid bitboard index: {}", index),
        };
        
        (piece_type, is_white)
    }
    
    /// Encodes a piece for the array representation
    fn encode_piece(&self, piece_type: u8, is_white: bool) -> u8 {
        let mut piece = PIECE_BIT + piece_type;
        if is_white {
            piece |= WHITE_BIT;
        }
        piece
    }
    
    /// Pops the least significant bit and returns its position
    fn pop_lsb(&self, bb: &mut u64) -> usize {
        let lsb_pos = bb.trailing_zeros() as usize;
        *bb &= *bb - 1; // Clear the LSB
        lsb_pos
    }
    
    /// Sets a piece on the board using bitboards
    pub fn set_piece_bitboard(&mut self, square: u8, piece_type: u8, is_white: bool) {
        let bitboard_index = self.get_bitboard_index(piece_type, is_white);
        self.bitboard[bitboard_index] |= 1u64 << square;
        self.state[square as usize] = self.encode_piece(piece_type, is_white);
    }
    
    /// Removes a piece from the board using bitboards
    pub fn remove_piece_bitboard(&mut self, square: u8) {
        let piece = self.state[square as usize];
        if piece != 0 {
            let is_white = (piece & WHITE_BIT) != 0;
            let piece_type = piece & 0b00001111;
            let bitboard_index = self.get_bitboard_index(piece_type, is_white);
            self.bitboard[bitboard_index] &= !(1u64 << square);
        }
        self.state[square as usize] = 0;
    }
    
    /// Moves a piece using bitboards
    pub fn move_piece_bitboard(&mut self, from: u8, to: u8) {
        let piece = self.state[from as usize];
        if piece != 0 {
            // Remove piece from old position
            self.remove_piece_bitboard(from);
            
            // Add piece to new position  
            let is_white = (piece & WHITE_BIT) != 0;
            let piece_type = piece & 0b00001111;
            self.set_piece_bitboard(to, piece_type, is_white);
        }
    }
    
    /// Gets all pieces of a specific color as a combined bitboard
    pub fn get_color_bitboard(&self, is_white: bool) -> u64 {
        let start_index = if is_white { 0 } else { 6 };
        let mut combined = 0u64;
        for i in start_index..start_index + 6 {
            combined |= self.bitboard[i];
        }
        combined
    }
    
    /// Gets all pieces on the board as a combined bitboard
    pub fn get_all_pieces_bitboard(&self) -> u64 {
        self.get_color_bitboard(true) | self.get_color_bitboard(false)
    }
    
    /// Gets attacks for a specific piece type at a square using bitboards
    pub fn get_piece_attacks(&self, square: u8, piece_type: u8, is_white: bool) -> u64 {
        match piece_type {
            t if t == PAWN_BIT => {
                if is_white {
                    masks::WHITE_PAWN_ATTACKS[square as usize]
                } else {
                    masks::BLACK_PAWN_ATTACKS[square as usize]
                }
            }
            t if t == KING => masks::KING_ATTACKS[square as usize],
            t if t == KNIGHT => masks::KNIGHT_ATTACKS[square as usize],
            t if t == ROOK => self.get_rook_attacks(square),
            t if t == BISHOP => self.get_bishop_attacks(square),
            t if t == QUEEN => self.get_rook_attacks(square) | self.get_bishop_attacks(square),
            _ => 0u64,
        }
    }
    
    /// Gets rook attacks using magic bitboards
    fn get_rook_attacks(&self, square: u8) -> u64 {
        let square_idx = square as usize;
        let mut blockers = self.get_all_pieces_bitboard() & masks::ROOK_MASKS[square_idx];
        blockers = blockers.wrapping_mul(masks::ROOK_MAGIC_NUMBERS[square_idx]);
        blockers >>= 64 - masks::ROOK_REL_BITS[square_idx];
        masks::ROOK_ATTACKS[square_idx][blockers as usize]
    }
    
    /// Gets bishop attacks using magic bitboards  
    fn get_bishop_attacks(&self, square: u8) -> u64 {
        let square_idx = square as usize;
        let mut blockers = self.get_all_pieces_bitboard() & masks::BISHOP_MASKS[square_idx];
        blockers = blockers.wrapping_mul(masks::BISHOP_MAGIC_NUMBERS[square_idx]);
        blockers >>= 64 - masks::BISHOP_REL_BITS[square_idx];
        masks::BISHOP_ATTACKS[square_idx][blockers as usize]
    }
    
    /// Bitboard-based bit scanning (find least significant bit)
    pub fn bitscan_forward(&self, bitboard: u64) -> usize {
        let bitboard_combined = bitboard ^ (bitboard - 1);
        let calculation = 0x03f79d71b4cb0a89u128 * bitboard_combined as u128;
        let calc_truncated = calculation as u64;
        let index = (calc_truncated >> 58) as usize;
        masks::DEBRUIJN64[index]
    }
    
    /// Pop least significant bit and return its position
    pub fn pop_lsb_bitboard(&self, bitboard: &mut u64) -> usize {
        let lsb_pos = self.bitscan_forward(*bitboard);
        *bitboard &= *bitboard - 1;
        lsb_pos
    }
}
