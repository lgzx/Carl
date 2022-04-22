#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod communicate;
mod config;
mod event;
mod search;

use communicate::register_handlers;
use core::str;
use event::Payload;
use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};
use std::thread;
use tauri::{App, AppHandle, Manager};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            register_handlers(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test(app: AppHandle) {
    let mut consumer = Consumer::from_hosts(vec!["kafka08-test.mars.ljnode.com:9092".to_owned()])
        .with_topic_partitions("beijia-private-domain-event-test".to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("ttestl1og122".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    let tapp_handler = app.clone();
    thread::spawn(move || loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}", std::str::from_utf8(m.value));
                tapp_handler
                    .emit_all(
                        "kafka-message",
                        Payload::new("test", str::from_utf8(m.value).unwrap().into()),
                    )
                    .unwrap()
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    });
}
