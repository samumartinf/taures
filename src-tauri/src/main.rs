use cherris::{self, engine::Engine, position_helper, ChessDebugInfo, ChessGame};
use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use rand::Rng;
use std::sync::{Arc, Mutex};

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

#[tauri::command]
fn play_move(source: &str, target: &str, promotion: &str) -> bool {
    let game = &mut ENGINE.lock().unwrap().game;

    game.play_move_from_string(source, target, promotion)
}

#[tauri::command]
fn restart_game() {
    let game = &mut ENGINE.lock().unwrap().game;
    game.restart();
}

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
fn get_fen_simple() -> String {
    let game = &mut ENGINE.lock().unwrap().game;
    game.get_fen_simple()
}

#[tauri::command]
fn get_piece_at_square(square: &str) -> String {
    let game = &mut ENGINE.lock().unwrap().game;
    game.get_piece_at_square(square.to_string())
}

#[tauri::command]
fn get_position_string() {
    let game = &mut ENGINE.lock().unwrap().game;
    game.board.show()
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

    game.play_move_ob(&random_move);
    game.get_fen_simple()
}

#[tauri::command]
fn play_best_move(depth: i32) -> String {
    println!("Playing best move with depth: {}", depth);
    let mut engine = ENGINE.lock().unwrap();
    let best_move = engine.get_best_move(depth as u8);
    let source_square = position_helper::index_to_letter(best_move.source);
    let target_square = position_helper::index_to_letter(best_move.target);
    println!("The best move was {} to {}", source_square, target_square);
    engine.game.play_move_ob(&best_move);
    engine.game.get_fen_simple()
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
            play_move,
            restart_game,
            undo_move,
            get_fen,
            get_fen_simple,
            get_piece_at_square,
            get_possible_moves,
            get_position_string,
            make_random_move,
            play_best_move,
            get_legal_moves,
            set_fen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
