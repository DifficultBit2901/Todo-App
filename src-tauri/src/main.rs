// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
mod macros;
mod project;

fn main() {
    setup();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            project::commands::get_all,
            project::commands::add,
            project::commands::delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup() {
    project::setup();
}
