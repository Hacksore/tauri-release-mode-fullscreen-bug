// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_custom::macos::WindowExtMacos;

mod window_custom;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      // we make the window higher than the league client and visible on all workspaces
      window.set_visisble_on_all_workspaces();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
