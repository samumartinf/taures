use cherris::engine::Engine;
use std::time::Instant;

fn main() {
    let mut engine = Engine::init();
    
    println!("Testing chess engine performance...");
    println!("Position: Starting position");
    
    // Test different depths
    for depth in 1..=4 {
        let start = Instant::now();
        let best_move = engine.get_best_move_optimized(depth);
        let elapsed = start.elapsed();
        
        println!("Depth {}: Best move from {} to {} in {:?} ({} positions, {:.0} pos/sec)",
            depth,
            cherris::position_helper::index_to_letter(best_move.source),
            cherris::position_helper::index_to_letter(best_move.target),
            elapsed,
            engine.num_positions_evaluated,
            engine.num_positions_evaluated as f64 / elapsed.as_secs_f64()
        );
        
        // Reset for next test
        engine.num_positions_evaluated = 0;
    }
}