#[cfg(test)]
mod tests {
    use crate::{Game, ChessGame};
    use std::time::Instant;

    #[test]
    fn test_move_generation_performance() {
        let mut game = Game::init();
        
        // Warm up
        for _ in 0..100 {
            let _ = game.get_all_moves_for_color(true);
            let _ = game.get_all_moves_bitboard(true);
        }
        
        let iterations = 10000;
        
        // Test traditional method
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = game.get_all_moves_for_color(true);
        }
        let traditional_time = start.elapsed();
        
        // Test bitboard method
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = game.get_all_moves_bitboard(true);
        }
        let bitboard_time = start.elapsed();
        
        println!("Performance comparison ({} iterations):", iterations);
        println!("Traditional method: {:?}", traditional_time);
        println!("Bitboard method: {:?}", bitboard_time);
        
        if traditional_time > bitboard_time {
            let speedup = traditional_time.as_secs_f64() / bitboard_time.as_secs_f64();
            println!("Bitboard method is {:.2}x faster", speedup);
        } else {
            let slowdown = bitboard_time.as_secs_f64() / traditional_time.as_secs_f64();
            println!("Traditional method is {:.2}x faster", slowdown);
        }
        
        // Both methods should be reasonably fast
        assert!(traditional_time.as_millis() < 5000);
        assert!(bitboard_time.as_millis() < 5000);
    }
}