#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::cell::RefCell;

struct Counter(RefCell<i32>);

unsafe impl Send for Counter {}
unsafe impl Sync for Counter {}

#[tauri::command]
fn counter(state: tauri::State<Counter>) {
  *state.0.borrow_mut() += 1;
}

#[tauri::command]
fn get_count(state: tauri::State<Counter>) -> i32 {
  *state.0.borrow()
}

fn main() {
  tauri::Builder::default()
  .manage(Counter(RefCell::new(0)))
    .invoke_handler(tauri::generate_handler![counter, get_count])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
