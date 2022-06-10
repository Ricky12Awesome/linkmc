#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::RwLock;

struct Counter(RwLock<i32>);

#[tauri::command]
fn counter(state: tauri::State<Counter>) -> i32 {
  *state.0.write().unwrap() += 1;
  *state.0.read().unwrap()
}

fn main() {
  tauri::Builder::default()
    .manage(Counter(RwLock::new(0)))
    .invoke_handler(tauri::generate_handler![counter])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
