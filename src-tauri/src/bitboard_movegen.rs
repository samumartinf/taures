use crate::board::Board;
use crate::masks;
use crate::{Move, BISHOP, KING, KNIGHT, PAWN_BIT, QUEEN, ROOK};

/// Fast bitboard-based move generation
pub struct BitboardMoveGen;

impl BitboardMoveGen {
    /// Pop least significant bit and return its position  
    fn pop_lsb(bitboard: &mut u64) -> usize {
        let lsb_pos = Self::bitscan_forward(*bitboard);
        *bitboard &= *bitboard - 1;
        lsb_pos
    }
    
    /// Bitboard-based bit scanning (find least significant bit)
    fn bitscan_forward(bitboard: u64) -> usize {
        let bitboard_combined = bitboard ^ (bitboard - 1);
        let calculation = 0x03f79d71b4cb0a89u128 * bitboard_combined as u128;
        let calc_truncated = calculation as u64;
        let index = (calc_truncated >> 58) as usize;
        masks::DEBRUIJN64[index]
    }
    /// Generate all pseudo-legal moves for a color using bitboards
    pub fn generate_moves(board: &Board, is_white: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        
        let (own_pieces, enemy_pieces) = if is_white {
            (board.get_color_bitboard(true), board.get_color_bitboard(false))
        } else {
            (board.get_color_bitboard(false), board.get_color_bitboard(true))
        };
        
        let all_pieces = own_pieces | enemy_pieces;
        let empty_squares = !all_pieces;
        
        // Generate moves for each piece type
        let piece_offset = if is_white { 0 } else { 6 };
        
        // Pawns
        Self::generate_pawn_moves(board, piece_offset, is_white, empty_squares, enemy_pieces, &mut moves);
        
        // Rooks
        Self::generate_sliding_moves(board, piece_offset + 1, ROOK, all_pieces, enemy_pieces, &mut moves);
        
        // Knights  
        Self::generate_knight_moves(board, piece_offset + 2, enemy_pieces, empty_squares, &mut moves);
        
        // Bishops
        Self::generate_sliding_moves(board, piece_offset + 3, BISHOP, all_pieces, enemy_pieces, &mut moves);
        
        // Queens
        Self::generate_sliding_moves(board, piece_offset + 4, QUEEN, all_pieces, enemy_pieces, &mut moves);
        
        // King
        Self::generate_king_moves(board, piece_offset + 5, enemy_pieces, empty_squares, &mut moves);
        
        moves
    }
    
    fn generate_pawn_moves(
        board: &Board, 
        piece_index: usize, 
        is_white: bool, 
        empty_squares: u64, 
        enemy_pieces: u64, 
        moves: &mut Vec<Move>
    ) {
        let mut pawns = board.bitboard[piece_index];
        
        while pawns != 0 {
            let from = Self::pop_lsb(&mut pawns);
            let from_u8 = from as u8;
            
            // Pawn pushes
            let forward_dir = if is_white { -8i8 } else { 8i8 };
            let one_square = (from as i8 + forward_dir) as usize;
            
            if one_square < 64 && (masks::SQUARE_BBS[one_square] & empty_squares) != 0 {
                // Check for promotion
                let promotion_rank = if is_white { 0 } else { 7 };
                if (one_square / 8) == promotion_rank {
                    // Add all promotion moves
                    moves.push(Move { source: from_u8, target: one_square as u8, promotion: Self::encode_promotion(QUEEN, is_white) });
                    moves.push(Move { source: from_u8, target: one_square as u8, promotion: Self::encode_promotion(ROOK, is_white) });
                    moves.push(Move { source: from_u8, target: one_square as u8, promotion: Self::encode_promotion(BISHOP, is_white) });
                    moves.push(Move { source: from_u8, target: one_square as u8, promotion: Self::encode_promotion(KNIGHT, is_white) });
                } else {
                    moves.push(Move { source: from_u8, target: one_square as u8, promotion: 0 });
                }
                
                // Double pawn push
                let start_rank = if is_white { 6 } else { 1 };
                if (from / 8) == start_rank {
                    let two_squares = (from as i8 + forward_dir * 2) as usize;
                    if two_squares < 64 && (masks::SQUARE_BBS[two_squares] & empty_squares) != 0 {
                        moves.push(Move { source: from_u8, target: two_squares as u8, promotion: 0 });
                    }
                }
            }
            
            // Pawn captures
            let attacks = if is_white {
                masks::WHITE_PAWN_ATTACKS[from]
            } else {
                masks::BLACK_PAWN_ATTACKS[from]
            };
            
            let mut captures = attacks & enemy_pieces;
            while captures != 0 {
                let to = Self::pop_lsb(&mut captures);
                let promotion_rank = if is_white { 0 } else { 7 };
                
                if (to / 8) == promotion_rank {
                    // Capture promotions
                    moves.push(Move { source: from_u8, target: to as u8, promotion: Self::encode_promotion(QUEEN, is_white) });
                    moves.push(Move { source: from_u8, target: to as u8, promotion: Self::encode_promotion(ROOK, is_white) });
                    moves.push(Move { source: from_u8, target: to as u8, promotion: Self::encode_promotion(BISHOP, is_white) });
                    moves.push(Move { source: from_u8, target: to as u8, promotion: Self::encode_promotion(KNIGHT, is_white) });
                } else {
                    moves.push(Move { source: from_u8, target: to as u8, promotion: 0 });
                }
            }
            
            // En passant
            if board.en_passant != 0 && (attacks & masks::SQUARE_BBS[board.en_passant as usize]) != 0 {
                moves.push(Move { source: from_u8, target: board.en_passant, promotion: 0 });
            }
        }
    }
    
    fn generate_knight_moves(
        board: &Board,
        piece_index: usize,
        enemy_pieces: u64,
        empty_squares: u64,
        moves: &mut Vec<Move>
    ) {
        let mut knights = board.bitboard[piece_index];
        
        while knights != 0 {
            let from = Self::pop_lsb(&mut knights);
            let attacks = masks::KNIGHT_ATTACKS[from];
            let mut targets = attacks & (enemy_pieces | empty_squares);
            
            while targets != 0 {
                let to = Self::pop_lsb(&mut targets);
                moves.push(Move { source: from as u8, target: to as u8, promotion: 0 });
            }
        }
    }
    
    fn generate_sliding_moves(
        board: &Board,
        piece_index: usize,
        piece_type: u8,
        all_pieces: u64,
        enemy_pieces: u64,
        moves: &mut Vec<Move>
    ) {
        let mut pieces = board.bitboard[piece_index];
        
        while pieces != 0 {
            let from = Self::pop_lsb(&mut pieces);
            
            let attacks = match piece_type {
                t if t == ROOK => Self::get_rook_attacks(from, all_pieces),
                t if t == BISHOP => Self::get_bishop_attacks(from, all_pieces),
                t if t == QUEEN => Self::get_rook_attacks(from, all_pieces) | Self::get_bishop_attacks(from, all_pieces),
                _ => 0u64,
            };
            
            let mut targets = attacks & (enemy_pieces | !all_pieces);
            
            while targets != 0 {
                let to = Self::pop_lsb(&mut targets);
                moves.push(Move { source: from as u8, target: to as u8, promotion: 0 });
            }
        }
    }
    
    fn generate_king_moves(
        board: &Board,
        piece_index: usize,
        enemy_pieces: u64,
        empty_squares: u64,
        moves: &mut Vec<Move>
    ) {
        let mut kings = board.bitboard[piece_index];
        
        while kings != 0 {
            let from = Self::pop_lsb(&mut kings);
            let attacks = masks::KING_ATTACKS[from];
            let mut targets = attacks & (enemy_pieces | empty_squares);
            
            while targets != 0 {
                let to = Self::pop_lsb(&mut targets);
                moves.push(Move { source: from as u8, target: to as u8, promotion: 0 });
            }
            
            // Add castling moves
            Self::generate_castling_moves(board, from, moves);
        }
    }
    
    fn get_rook_attacks(square: usize, occupancy: u64) -> u64 {
        let mut blockers = occupancy & masks::ROOK_MASKS[square];
        blockers = blockers.wrapping_mul(masks::ROOK_MAGIC_NUMBERS[square]);
        blockers >>= 64 - masks::ROOK_REL_BITS[square];
        masks::ROOK_ATTACKS[square][blockers as usize]
    }
    
    fn get_bishop_attacks(square: usize, occupancy: u64) -> u64 {
        let mut blockers = occupancy & masks::BISHOP_MASKS[square];
        blockers = blockers.wrapping_mul(masks::BISHOP_MAGIC_NUMBERS[square]);
        blockers >>= 64 - masks::BISHOP_REL_BITS[square];
        masks::BISHOP_ATTACKS[square][blockers as usize]
    }
    
    fn encode_promotion(piece_type: u8, is_white: bool) -> u8 {
        use crate::{PIECE_BIT, WHITE_BIT};
        let mut piece = PIECE_BIT + piece_type;
        if is_white {
            piece |= WHITE_BIT;
        }
        piece
    }
    
    /// Check if a square is attacked by the given color
    pub fn is_square_attacked(board: &Board, square: u8, by_white: bool) -> bool {
        let attacker_offset = if by_white { 0 } else { 6 };
        let all_pieces = board.get_all_pieces_bitboard();
        let square_idx = square as usize;
        
        // Check pawn attacks
        let pawn_attacks = if by_white {
            masks::BLACK_PAWN_ATTACKS[square_idx] // White pawns attack from black pawn attack squares
        } else {
            masks::WHITE_PAWN_ATTACKS[square_idx] // Black pawns attack from white pawn attack squares
        };
        if (board.bitboard[attacker_offset] & pawn_attacks) != 0 {
            return true;
        }
        
        // Check knight attacks
        if (board.bitboard[attacker_offset + 2] & masks::KNIGHT_ATTACKS[square_idx]) != 0 {
            return true;
        }
        
        // Check king attacks
        if (board.bitboard[attacker_offset + 5] & masks::KING_ATTACKS[square_idx]) != 0 {
            return true;
        }
        
        // Check sliding piece attacks
        let rook_attacks = Self::get_rook_attacks(square_idx, all_pieces);
        if (board.bitboard[attacker_offset + 1] & rook_attacks) != 0 || // Rooks
           (board.bitboard[attacker_offset + 4] & rook_attacks) != 0    // Queens
        {
            return true;
        }
        
        let bishop_attacks = Self::get_bishop_attacks(square_idx, all_pieces);
        if (board.bitboard[attacker_offset + 3] & bishop_attacks) != 0 || // Bishops
           (board.bitboard[attacker_offset + 4] & bishop_attacks) != 0    // Queens
        {
            return true;
        }
        
        false
    }
    
    fn generate_castling_moves(board: &Board, king_square: usize, moves: &mut Vec<Move>) {
        let is_white = king_square == 60; // e1 for white, e8 for black
        let king_square_u8 = king_square as u8;
        
        if is_white {
            // White castling
            if (board.castling & 8u8) != 0 && king_square == 60 { // KQkq format, bit 3 = white kingside
                // Check kingside castling (e1-g1)
                let all_pieces = board.get_all_pieces_bitboard();
                if (all_pieces & 0x60) == 0 { // f1 and g1 empty
                    // Check if king, f1, g1 are not attacked
                    if !Self::is_square_attacked(board, 60, false) && // e1
                       !Self::is_square_attacked(board, 61, false) && // f1
                       !Self::is_square_attacked(board, 62, false) {  // g1
                        moves.push(Move { source: king_square_u8, target: 62, promotion: 0 });
                    }
                }
            }
            
            if (board.castling & 4u8) != 0 && king_square == 60 { // bit 2 = white queenside
                // Check queenside castling (e1-c1)
                let all_pieces = board.get_all_pieces_bitboard();
                if (all_pieces & 0xE) == 0 { // b1, c1, d1 empty
                    // Check if king, d1, c1 are not attacked
                    if !Self::is_square_attacked(board, 60, false) && // e1
                       !Self::is_square_attacked(board, 59, false) && // d1
                       !Self::is_square_attacked(board, 58, false) {  // c1
                        moves.push(Move { source: king_square_u8, target: 58, promotion: 0 });
                    }
                }
            }
        } else {
            // Black castling
            if (board.castling & 2u8) != 0 && king_square == 4 { // bit 1 = black kingside
                // Check kingside castling (e8-g8)
                let all_pieces = board.get_all_pieces_bitboard();
                if (all_pieces & 0x6000000000000000) == 0 { // f8 and g8 empty
                    // Check if king, f8, g8 are not attacked
                    if !Self::is_square_attacked(board, 4, true) && // e8
                       !Self::is_square_attacked(board, 5, true) && // f8
                       !Self::is_square_attacked(board, 6, true) {  // g8
                        moves.push(Move { source: king_square_u8, target: 6, promotion: 0 });
                    }
                }
            }
            
            if (board.castling & 1u8) != 0 && king_square == 4 { // bit 0 = black queenside
                // Check queenside castling (e8-c8)
                let all_pieces = board.get_all_pieces_bitboard();
                if (all_pieces & 0xE00000000000000) == 0 { // b8, c8, d8 empty
                    // Check if king, d8, c8 are not attacked
                    if !Self::is_square_attacked(board, 4, true) && // e8
                       !Self::is_square_attacked(board, 3, true) && // d8
                       !Self::is_square_attacked(board, 2, true) {  // c8
                        moves.push(Move { source: king_square_u8, target: 2, promotion: 0 });
                    }
                }
            }
        }
    }
}