use color_eyre::eyre::Result;
use cherris::{self, ChessGame};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut board: cherris::Board = cherris::Board::init();
    board.update_hashmap();

    let mut game = cherris::Game::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}