// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
pub mod structure;
pub mod table_creation;

#[tauri::command]
fn generate_new_table(height: &str, width: &str) -> String {
    println!("width: {} height: {}", width, height);
    table_creation::new_table(height, width)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_new_table])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
