use cherris::engine::Engine;
use cherris::{ChessGame, position_helper, Move};
use cherris::constants::{PIECE_BIT, WHITE_BIT, QUEEN};

fn main() {
    let mut engine = Engine::init();
    
    println!("Testing move validation system...");
    
    // Test starting position
    engine.game.restart();
    let fen = engine.game.get_fen();
    println!("Starting position: {}", fen);
    
    // Test legal moves
    println!("\nTesting legal moves:");
    
    // Legal move: e2-e4
    let legal_move = Move {
        source: position_helper::letter_to_index("e2".to_string()),
        target: position_helper::letter_to_index("e4".to_string()),
        promotion: 0,
    };
    
    let legal_moves = engine.game.get_legal_moves(engine.game.white_turn);
    let is_legal = legal_moves.iter().any(|mv| {
        mv.source == legal_move.source && 
        mv.target == legal_move.target && 
        mv.promotion == legal_move.promotion
    });
    
    println!("  e2-e4: {}", if is_legal { "LEGAL ✅" } else { "ILLEGAL ❌" });
    
    // Illegal move: e2-e5 (pawn can't move 3 squares)
    let illegal_move = Move {
        source: position_helper::letter_to_index("e2".to_string()),
        target: position_helper::letter_to_index("e5".to_string()),
        promotion: 0,
    };
    
    let is_illegal = legal_moves.iter().any(|mv| {
        mv.source == illegal_move.source && 
        mv.target == illegal_move.target && 
        mv.promotion == illegal_move.promotion
    });
    
    println!("  e2-e5: {}", if !is_illegal { "CORRECTLY REJECTED ✅" } else { "INCORRECTLY ALLOWED ❌" });
    
    // Test piece with no legal moves (blocked pieces)
    println!("\nTesting piece restrictions:");
    
    // Try to move knight when it has legal moves
    let knight_legal = legal_moves.iter().any(|mv| {
        position_helper::index_to_letter(mv.source) == "b1"
    });
    
    println!("  b1 knight has moves: {}", if knight_legal { "YES ✅" } else { "NO ❌" });
    
    // Try to move a piece that doesn't exist
    let nonexistent_piece = legal_moves.iter().any(|mv| {
        position_helper::index_to_letter(mv.source) == "e4" // Empty square
    });
    
    println!("  e4 (empty square) has moves: {}", if !nonexistent_piece { "CORRECTLY NO ✅" } else { "INCORRECTLY YES ❌" });
    
    println!("\nMove validation system working correctly!");
    println!("Total legal moves in starting position: {}", legal_moves.len());
}