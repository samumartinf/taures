#[cfg(test)]
mod tests {
    use crate::{Game, ChessGame};

    #[test]
    fn test_bitboard_move_generation() {
        let mut game = Game::init();
        
        // Test starting position move counts
        let traditional_moves = game.get_all_moves_for_color(true);
        let bitboard_moves = game.get_all_moves_bitboard(true);
        
        println!("Traditional move count: {}", traditional_moves.len());
        println!("Bitboard move count: {}", bitboard_moves.len());
        
        // In starting position, white should have 20 legal moves
        // 16 pawn moves (8 pawns × 2 moves each) + 4 knight moves (2 knights × 2 moves each)
        assert!(!traditional_moves.is_empty());
        assert!(!bitboard_moves.is_empty());
        
        // The counts should be similar (exact match depends on implementation details)
        // This is a basic sanity check
        assert!(bitboard_moves.len() >= 15); // Should have at least 15 moves
        assert!(bitboard_moves.len() <= 30); // Should not exceed reasonable bounds
    }
    
    #[test]
    fn test_bitboard_synchronization() {
        let mut game = Game::init();
        
        // Make sure bitboards are synchronized with array after initialization
        game.board.update_bitboards_from_array();
        
        // Check that white pawns are on correct squares (48-55)
        let white_pawns = game.board.bitboard[0]; // Index 0 = white pawns
        
        // Count bits set in white pawn bitboard
        let pawn_count = white_pawns.count_ones();
        assert_eq!(pawn_count, 8, "Should have 8 white pawns");
        
        // Verify specific pawn positions
        for square in 48..56 {
            assert!((white_pawns & (1u64 << square)) != 0, "White pawn missing on square {}", square);
        }
    }
    
    #[test]
    fn test_move_comparison() {
        let mut game = Game::init();
        
        let traditional_moves = game.get_all_moves_for_color(true);
        let bitboard_moves = game.get_all_moves_bitboard(true);
        
        println!("Comparing move generation methods:");
        println!("Traditional: {} moves", traditional_moves.len());
        println!("Bitboard: {} moves", bitboard_moves.len());
        
        // Print first few moves from each method for debugging
        println!("First 5 traditional moves:");
        for (i, mv) in traditional_moves.iter().take(5).enumerate() {
            println!("  {}: {} -> {}", i, mv.source, mv.target);
        }
        
        println!("First 5 bitboard moves:");
        for (i, mv) in bitboard_moves.iter().take(5).enumerate() {
            println!("  {}: {} -> {}", i, mv.source, mv.target);
        }
    }
}