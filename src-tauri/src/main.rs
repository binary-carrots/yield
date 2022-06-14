#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn quit_app() {
    std::process::exit(0);
}

#[tauri::command]
fn send_message() {
    println!("this is a message sent from the front end");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![quit_app, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
