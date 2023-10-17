// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, MouseControllable};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_mouse_pos() -> (i32, i32) {
    let enigo = Enigo::new();

    return enigo.mouse_location();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_mouse_pos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
