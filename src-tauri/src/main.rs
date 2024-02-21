use cherris::{self, engine::Engine, position_helper, ChessDebugInfo, ChessGame, Game};
use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use rand::Rng;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref GAME: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game::init()));
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn set_from_fen(fen: &str) -> String {
    let mut game = GAME.lock().unwrap();
    game.set_from_fen(fen.to_string());
    game.get_fen()
}

#[tauri::command]
fn play_move(source: &str, target: &str) -> bool {
    let mut game = GAME.lock().unwrap();
    
    game.play_move_from_string(source.to_string(), target.to_string())
}

#[tauri::command]
fn restart_game() {
    let mut game = GAME.lock().unwrap();
    game.restart();
}

#[tauri::command]
fn undo_move() {
    let mut game = GAME.lock().unwrap();
    game.undo_move();
}

#[tauri::command]
fn get_possible_moves(source: &str) -> Vec<String> {
    let game = GAME.lock().unwrap();
    game.get_pseudolegal_moves(source.to_string())
}

#[tauri::command]
fn get_fen() -> String {
    let game = GAME.lock().unwrap();
    game.get_fen()
}

#[tauri::command]
fn get_fen_simple() -> String {
    let game = GAME.lock().unwrap();
    game.get_fen_simple()
}

#[tauri::command]
fn get_piece_at_square(square: &str) -> String {
    let game = GAME.lock().unwrap();
    game.get_piece_at_square(square.to_string())
}

#[tauri::command]
fn get_position_string() {
    let game = GAME.lock().unwrap();
    game.board.show()
}

#[tauri::command]
fn make_random_move() -> String {
    let mut game = GAME.lock().unwrap();
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
    let mut game = GAME.lock().unwrap();
    let mut engine = Engine::init_from_game(game.clone());
    let best_move = engine.get_best_move(depth as u8);
    let source_square = position_helper::index_to_letter(best_move.source);
    let target_square = position_helper::index_to_letter(best_move.target);
    println!("The best move was {} to {}", source_square, target_square);
    game.play_move_ob(&best_move);
    game.get_fen_simple()
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
