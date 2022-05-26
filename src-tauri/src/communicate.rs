use std::{
    collections::{BTreeMap, HashMap},
    io::{Cursor, Error},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use app::pb::Response;
use app::{command::Execute, pb::Request, server::KafkaServer};
use prost::{DecodeError, Message};
use tauri::{App, Manager};

pub fn register_handlers(app: &mut App) {
    let srv = Arc::new(Mutex::new(KafkaServer::new()));
    let srv_clone = srv.clone();
    let mut window = app.get_window("main").unwrap();
    app.app_handle().listen_global("event", move |e| {
        let binary: BTreeMap<u32, u8> = serde_json::from_str(e.payload().unwrap()).unwrap();
        let byte_info = binary.values().cloned().collect::<Vec<u8>>();
        let req: Result<Request, DecodeError> = Message::decode(&mut Cursor::new(&byte_info));
        let s: &mut KafkaServer = &mut srv_clone.lock().unwrap();

        match &req {
            Ok(r) => {
                match accept(r, s) {
                    Ok(v) => {
                        println!("response: {:?}", v);
                        window.emit("event-back", v);
                    }
                    Err(e) => println!("Error: {:?}", e),
                };
            }
            Err(e) => {
                println!("error {:?}", e);
            }
        }
    });
    drop(srv);
}

fn accept(req: &Request, srv: &mut KafkaServer) -> Result<Response> {
    req.execute(srv)
}
