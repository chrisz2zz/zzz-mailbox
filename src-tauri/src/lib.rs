use std::env;

use db::init_db;

mod config;
mod db;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn welcome() -> String {
    format!("xxxxxx")
}

#[tauri::command]
fn get_email(last: i32) -> Vec<String> {
    let _last = last;
    vec![
        "xxxx1".to_owned(),
        "xxxx2".to_owned(),
        "xxxx3".to_owned(),
        "xxxx4".to_owned(),
        "xxxx5".to_owned(),
        "xxxx6".to_owned(),
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_db(&mut env::current_dir().unwrap()).expect("error while running tauri application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, welcome, get_email])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
