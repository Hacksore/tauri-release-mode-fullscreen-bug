// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod window_custom;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      window_custom::macos::WindowExtMacos::set_visisble_on_all_workspaces(&window, true);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
