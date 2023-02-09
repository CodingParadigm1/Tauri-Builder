#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod projects;
use projects::manage::{get_dir, generate_tauri_app, open_vscode};  


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dir, generate_tauri_app, open_vscode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
