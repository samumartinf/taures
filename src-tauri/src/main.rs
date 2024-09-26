use cherris::{self, engine::Engine, position_helper, ChessDebugInfo, ChessGame, Move};
use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use rand::Rng;
use std::sync::{Arc, Mutex};

use cherris::constants::{ PIECE_BIT, WHITE_BIT, QUEEN };

lazy_static! {
    static ref ENGINE: Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::init()));
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn set_from_fen(fen: &str) -> String {
    let game = &mut ENGINE.lock().unwrap().game;
    game.set_from_fen(fen.to_string());
    game.get_fen()
}


fn play_move(source: &str, target: &str, promotion: &str) -> String {
    println!("We want to move from {} to {}.", source, target);
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

    let is_legal = game.play_move_ob(move_obj);

    println!("The move legality was {}", is_legal);
    // get the FEN String
    let fen = game.get_fen_simple();
    fen
}

#[tauri::command]
fn restart_game() {
    let game = &mut ENGINE.lock().unwrap().game;
    game.restart();
}


// TODO: We shoud return the FEN here
#[tauri::command]
fn undo_move() {
    let game = &mut ENGINE.lock().unwrap().game;
    game.undo_move();
}

#[tauri::command]
fn get_possible_moves(source: &str) -> Vec<String> {
    let game = &mut ENGINE.lock().unwrap().game;
    game.undo_move();
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
    println!("The FEN was: {}", fen);
    return fen;
}

#[tauri::command]
fn get_engine_move(depth: i32) -> String {
    println!("Playing best move with depth: {}", depth);
    let mut engine = ENGINE.lock().unwrap();
    let best_move = engine.get_best_move(depth as u8);
    let source_square = position_helper::index_to_letter(best_move.source);
    let target_square = position_helper::index_to_letter(best_move.target);
    println!("The best move was {} to {}", source_square, target_square);
    engine.game.play_move_ob(best_move);
    engine.game.get_fen()
}

#[tauri::command]
fn get_legal_moves(source: &str) -> Vec<String> {
    let game = &mut ENGINE.lock().unwrap().game;
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
