#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod communicate;

use app::server::KafkaServer;
use communicate::register_handlers;
use tauri::{Manager, WindowBuilder, WindowUrl};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            register_handlers(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
