use core::str;
use std::thread;

use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};
use tauri::{App, Manager};

use crate::event::Payload;

pub fn register_handlers(app: &mut App) {
    app.app_handle()
        .listen_global("test-event", |evt| println!("{:?}", evt));
}

fn send_kafka(app: &mut App) {}
