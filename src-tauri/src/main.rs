#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use curseforge::endpoints::DEFAULT_API_BASE;
use curseforge::types::Project;
use tauri::async_runtime::RwLock;
use tauri::State;

struct ProjectState(RwLock<Option<Project>>);

type Result<T> = std::result::Result<T, String>;

#[tauri::command]
async fn test(state: State<'_, ProjectState>) -> Result<Project> {
  let value = state.0.read().await.clone();

  return match value {
    Some(project) => Ok(project),
    None => {
      const TOKEN: &str = env!("CF_API");

      let client = curseforge::Client::new(DEFAULT_API_BASE, Some(TOKEN.to_string())).unwrap();
      let project = client.project(363581).await.unwrap();

      *state.0.write().await = Some(project.clone());

      Ok(project)
    }
  };
}

#[tauri::command]
fn open() -> Option<std::path::PathBuf> {
  native_dialog::FileDialog::new()
    .show_open_single_dir()
    .ok()
    .flatten()
}

fn main() {
  tauri::Builder::default()
    .manage(ProjectState(RwLock::default()))
    .invoke_handler(tauri::generate_handler![test, open])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
