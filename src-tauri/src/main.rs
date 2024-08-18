// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fix_path_env::fix;

fn main() {
if let Err(e) = fix() {
    println!("{}", e);
  } else {
    tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  }
  
}
