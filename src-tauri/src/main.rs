#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

const URL: &str = "https://pokerogue.net/";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        // 웹뷰 사용
        .setup(|app| {
            let wind = app.get_window("main").unwrap();
            let _ = wind.eval(&format!("window.location.replace('{}')", URL));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
