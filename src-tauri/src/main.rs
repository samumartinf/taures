use cherris::{self, ChessDebugInfo, ChessGame, Game};
use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref GAME: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game::init()));
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn set_from_fen(fen: &str) -> String {
    let mut game = GAME.lock().unwrap();
    game.set_from_fen(fen.to_string());
    return game.get_fen();
}

#[tauri::command]
fn play_move(source: &str, target: &str) -> bool {
    let mut game = GAME.lock().unwrap();
    let allowed_move = game.play_move_from_string(source.to_string(), target.to_string());
    return allowed_move;
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
    return game.get_pseudolegal_moves(source.to_string());
}

#[tauri::command]
fn get_fen() -> String {
    let game = GAME.lock().unwrap();
    return game.get_fen();
}

#[tauri::command]
fn get_fen_simple() -> String {
    let game = GAME.lock().unwrap();
    return game.get_fen_simple();
}

#[tauri::command]
fn get_piece_at_square(square: &str) -> String {
    let game = GAME.lock().unwrap();
    return game.get_piece_at_square(square.to_string());
}

#[tauri::command]
fn get_position_string() {
    let game = GAME.lock().unwrap();
    return game.board.show();
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut board: cherris::Board = cherris::Board::init();
    board.update_hashmap();

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
