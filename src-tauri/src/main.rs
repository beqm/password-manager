// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn sum(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, sum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
