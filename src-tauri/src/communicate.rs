use tauri::{App, Event, Manager};

pub fn register_handlers(app: &mut App) {
    app.app_handle()
        .listen_global("config:kafka:list_configs", list_config);
}

fn list_config(evt: Event) {}
