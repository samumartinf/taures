use cherris::{self, engine::Engine, position_helper, ChessDebugInfo, ChessGame, Move};
use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use rand::Rng;
use serde::Serialize;
use std::sync::{Arc, Mutex};

use cherris::constants::{ PIECE_BIT, WHITE_BIT, QUEEN };

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
        let start = Instant::now();
        
        // Use the optimized engine for better performance
        let best_move = engine.get_best_move_optimized(depth as u8);
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
    for m in moves {
        if position_helper::index_to_letter(m.source) == source {
            result.push(position_helper::index_to_letter(m.target));
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
            is_engine_computing
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
