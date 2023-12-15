// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
pub mod structure;
// use structure;

#[tauri::command]
fn generate_new_table(height: &str, width: &str) -> String {
    println!("width: {} height: {}", width, height);
    let height: usize = match height.parse() {
        Ok(number) => number,
        Err(_) => todo!("only numbers plz"),
    };
    let width: usize = match width.parse() {
        Ok(number) => number,
        Err(_) => todo!("only numbers plz"),
    };
    let table = structure::Table::new(height, width);
    table.to_html()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_new_table])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
