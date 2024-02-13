// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::settings::AppSettings;

pub mod tests;
mod api;
mod gui;
mod atol;
mod settings;


fn main() {
    let _ = AppSettings::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![gui::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
