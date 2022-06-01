#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod command;
pub mod communicate;
pub mod config;
pub mod handler;
pub mod kafka;
pub mod pb;
pub mod server;
pub mod shortcut;

use std::sync::{Arc, Mutex};

use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use server::{KafkaServer, ServerState};
use tauri::{generate_handler, CustomMenuItem, Menu, MenuItem, Runtime, Submenu, Window};

use communicate::register_handlers;
use tauri::{App, Manager, PhysicalSize, Size, WindowBuilder, WindowUrl};

fn main() {
    let submenu = Submenu::new(
        "File",
        Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::SelectAll),
    );
    let menu = Menu::new().add_submenu(submenu);
    let srv = Arc::new(Mutex::new(KafkaServer::new()));

    tauri::Builder::default()
        .menu(menu)
        .setup(move |app| {
            let srv_clone = srv.clone();
            let manager_state = srv.clone();
            app.app_handle().manage(ServerState {
                state: manager_state,
            });
            init_window(app);
            register_handlers(app, srv_clone);
            Ok(())
        })
        .invoke_handler(generate_handler![handler::run_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_window(app: &mut App) {
    let window_size = PhysicalSize {
        width: 1200,
        height: 900,
    };
    let window = app.get_window("main").unwrap();
    window.set_size(Size::Physical(window_size)).unwrap();
    window
        .set_min_size(Some(Size::Physical(window_size)))
        .unwrap();
    window.set_transparent_titlebar(false);
}

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool) {
        use cocoa::appkit::NSWindowTitleVisibility;

        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;

            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                transparent,
            );
            id.setStyleMask_(style_mask);

            id.setTitleVisibility_(if transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });
            id.setTitlebarAppearsTransparent_(if transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}
