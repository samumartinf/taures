use cherris::{self, engine::Engine, position_helper, ChessDebugInfo, ChessGame, Move};
use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use rand::Rng;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use cherris::constants::{ PIECE_BIT, WHITE_BIT, QUEEN };

// Function to get a random opening move from the book
fn get_opening_book_move(fen: &str) -> Option<Move> {
    if let Some(moves) = OPENING_BOOK.get(fen) {
        if !moves.is_empty() {
            let mut rng = rand::thread_rng();
            let random_move = moves[rng.gen_range(0..moves.len())];
            
            // Parse the move string (e.g., "e2e4" -> Move struct)
            if random_move.len() >= 4 {
                let from_square = &random_move[0..2];
                let to_square = &random_move[2..4];
                
                return Some(Move {
                    source: position_helper::letter_to_index(from_square.to_string()),
                    target: position_helper::letter_to_index(to_square.to_string()),
                    promotion: 0, // No promotion in opening moves
                });
            }
        }
    }
    None
}

#[derive(Serialize)]
struct EngineResponse {
    fen: String,
    positions_evaluated: i64,
    time_ms: u128,
    positions_per_second: f64,
    best_move: String,
}

lazy_static! {
    static ref ENGINE: Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::init()));
    static ref ENGINE_COMPUTING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref USE_OPENING_BOOK: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    static ref OPENING_BOOK: HashMap<&'static str, Vec<&'static str>> = {
        let mut book = HashMap::new();
        
        // Starting position - popular first moves for white
        book.insert("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![
            "e2e4",   // King's Pawn
            "d2d4",   // Queen's Pawn  
            "g1f3",   // Reti Opening
            "c2c4",   // English Opening
            "b1c3",   // Van't Kruijs Opening
            "f2f4",   // Bird's Opening
            "b2b3",   // Nimzowitsch-Larsen Attack
            "g2g3"    // King's Indian Attack
        ]);
        
        // After 1.e4 - popular black responses
        book.insert("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1", vec![
            "e7e5",   // King's Pawn Game
            "c7c5",   // Sicilian Defense
            "e7e6",   // French Defense
            "c7c6",   // Caro-Kann Defense
            "d7d6",   // Pirc Defense
            "g8f6",   // Alekhine's Defense
            "b8c6",   // Nimzowitsch Defense
            "d7d5"    // Scandinavian Defense
        ]);
        
        // After 1.d4 - popular black responses  
        book.insert("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1", vec![
            "d7d5",   // Queen's Gambit
            "g8f6",   // Indian Defenses
            "f7f5",   // Dutch Defense
            "e7e6",   // French-style setup
            "c7c5",   // Benoni Defense
            "g7g6",   // Modern Defense
            "b8c6",   // Mikenas Defense
            "e7e5"    // Englund Gambit
        ]);
        
        // After 1.Nf3 - popular black responses
        book.insert("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1", vec![
            "d7d5",   // Queen's Pawn
            "g8f6",   // Symmetrical
            "c7c5",   // English Defense
            "e7e6",   // Flexible setup
            "g7g6",   // King's Indian setup
            "b8c6",   // Knight development
            "f7f5",   // Dutch-style
            "d7d6"    // Pirc-style
        ]);
        
        book
    };
}

#[tauri::command]
fn set_from_fen(fen: &str) -> String {
    let game = &mut ENGINE.lock().unwrap().game;
    game.set_from_fen(fen.to_string());
    game.get_fen()
}

#[tauri::command]
fn play_move(source: &str, target: &str, promotion: &str) -> String {
    println!("Playing move from {} to {} with promotion {}", source, target, promotion);
    let game = &mut ENGINE.lock().unwrap().game;


    // Force promotion to a queen for now 
    let promotion_piece = match promotion {
        "Q" => PIECE_BIT + WHITE_BIT + QUEEN,
        "q" => PIECE_BIT + QUEEN,
            _ => 0,
        };

    let move_obj = Move {
        source: position_helper::letter_to_index(source.to_string()),
        target: position_helper::letter_to_index(target.to_string()),
        promotion: promotion_piece,
    };

    game.play_move_ob(move_obj);

    // get the FEN String
    let fen = game.get_fen_simple();
    fen
}

#[tauri::command]
fn restart_game() {
    println!("Restarting game");
    let game = &mut ENGINE.lock().unwrap().game;
    game.restart();
}


// TODO: We shoud return the FEN here
#[tauri::command]
fn undo_move() -> String {
    println!("Undoing move");
    let game = &mut ENGINE.lock().unwrap().game;
    game.undo_move();
    game.get_fen()
}

#[tauri::command]
fn get_possible_moves(source: &str) -> Vec<String> {
    let game = &mut ENGINE.lock().unwrap().game;
    game.get_pseudolegal_moves(source.to_string())
}

#[tauri::command]
fn get_fen() -> String {
    let game = &mut ENGINE.lock().unwrap().game;
    game.get_fen()
}


#[tauri::command]
fn get_piece_at_square(square: &str) -> String {
    let game = &mut ENGINE.lock().unwrap().game;
    game.get_piece_at_square(square.to_string())
}


#[tauri::command]
fn make_random_move() -> String {
    let game = &mut ENGINE.lock().unwrap().game;
    let moves = game.get_legal_moves(game.white_turn);

    if moves.is_empty() {
        println!("No legal moves available");
        return "None".to_string();
    }

    // select a random move
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..moves.len());
    let random_move = moves[random_index];

    game.play_move_ob(random_move);
    let fen = game.get_fen();
    return fen;
}

#[tauri::command]
async fn get_engine_move(depth: i32) -> Result<EngineResponse, String> {
    // Check if engine is already computing
    {
        let mut computing = ENGINE_COMPUTING.lock().unwrap();
        if *computing {
            println!("Engine is already computing, ignoring request");
            return Err("COMPUTING".to_string());
        }
        *computing = true;
    }
    
    println!("Playing best move with depth: {}", depth);
    
    // Clone the Arc to move into the blocking task
    let engine_arc = ENGINE.clone();
    let computing_arc = ENGINE_COMPUTING.clone();
    
    // Run the CPU-intensive computation in a separate thread
    let result = tokio::task::spawn_blocking(move || {
        use std::time::Instant;
        
        let mut engine = engine_arc.lock().unwrap();
        let current_fen = engine.game.get_fen();
        let start = Instant::now();
        
        // First check if we should use opening book and have a move available
        let best_move = if *USE_OPENING_BOOK.lock().unwrap() {
            if let Some(opening_move) = get_opening_book_move(&current_fen) {
                println!("Using opening book move!");
                opening_move
            } else {
                // Fall back to engine calculation
                engine.get_best_move_optimized(depth as u8)
            }
        } else {
            // Use pure engine calculation
            engine.get_best_move_optimized(depth as u8)
        };
        
        let elapsed = start.elapsed();
        
        let source_square = position_helper::index_to_letter(best_move.source);
        let target_square = position_helper::index_to_letter(best_move.target);
        let best_move_str = format!("{}-{}", source_square, target_square);
        
        println!("The best move was {} to {} in {:?}", source_square, target_square, elapsed);
        
        engine.game.play_move_ob(best_move);
        let fen = engine.game.get_fen();
        
        let positions_evaluated = engine.num_positions_evaluated;
        let time_ms = elapsed.as_millis();
        let positions_per_second = if time_ms > 0 {
            (positions_evaluated as f64) / (time_ms as f64 / 1000.0)
        } else {
            0.0
        };
        
        // Reset the computing flag
        *computing_arc.lock().unwrap() = false;
        
        EngineResponse {
            fen,
            positions_evaluated,
            time_ms,
            positions_per_second,
            best_move: best_move_str,
        }
    }).await;
    
    match result {
        Ok(response) => Ok(response),
        Err(e) => {
            eprintln!("Error in engine computation: {}", e);
            // Make sure to reset the flag on error
            *ENGINE_COMPUTING.lock().unwrap() = false;
            Err("Error in engine computation".to_string())
        }
    }
}

#[tauri::command]
fn get_legal_moves(source: &str) -> Vec<String> {
    let game: &mut cherris::Game = &mut ENGINE.lock().unwrap().game;
    let moves = game.get_legal_moves(game.white_turn);
    let mut result = Vec::new();
    
    if source == "all" {
        // Return all legal moves in "source-target" format
        for m in moves {
            let source_square = position_helper::index_to_letter(m.source);
            let target_square = position_helper::index_to_letter(m.target);
            result.push(format!("{}-{}", source_square, target_square));
        }
    } else {
        // Return only target squares for a specific source
        for m in moves {
            if position_helper::index_to_letter(m.source) == source {
                result.push(position_helper::index_to_letter(m.target));
            }
        }
    }
    result
}

#[tauri::command]
fn set_fen(fen: &str) -> bool {
    let game = &mut ENGINE.lock().unwrap().game;
    game.set_from_simple_fen(fen.to_string())
}

#[tauri::command]
fn is_engine_computing() -> bool {
    *ENGINE_COMPUTING.lock().unwrap()
}

#[tauri::command]
fn set_opening_variety(enabled: bool) {
    *USE_OPENING_BOOK.lock().unwrap() = enabled;
    println!("Opening variety {}", if enabled { "enabled" } else { "disabled" });
}

#[tauri::command]
fn get_opening_variety() -> bool {
    *USE_OPENING_BOOK.lock().unwrap()
}

#[tauri::command]
fn is_move_legal(source: &str, target: &str, promotion: &str) -> bool {
    let game = &mut ENGINE.lock().unwrap().game;
    
    // Force promotion to a queen for now 
    let promotion_piece = match promotion {
        "Q" => PIECE_BIT + WHITE_BIT + QUEEN,
        "q" => PIECE_BIT + QUEEN,
        _ => 0,
    };

    let move_obj = Move {
        source: position_helper::letter_to_index(source.to_string()),
        target: position_helper::letter_to_index(target.to_string()),
        promotion: promotion_piece,
    };
    
    // Get all legal moves for current player
    let legal_moves = game.get_legal_moves(game.white_turn);
    
    // Check if the proposed move matches any legal move exactly
    legal_moves.iter().any(|legal_move| {
        legal_move.source == move_obj.source && 
        legal_move.target == move_obj.target && 
        legal_move.promotion == move_obj.promotion
    })
}

fn main() -> Result<()> {
    color_eyre::install()?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_from_fen,
            restart_game,
            undo_move,
            get_fen,
            get_piece_at_square,
            get_possible_moves,
            make_random_move,
            get_engine_move,
            get_legal_moves,
            set_fen,
            play_move,
            is_engine_computing,
            set_opening_variety,
            get_opening_variety,
            is_move_legal
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
