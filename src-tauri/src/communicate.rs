use std::{
    collections::{BTreeMap, HashMap},
    io::{Cursor, Error},
    sync::{Arc, Mutex},
};

use crate::{command::Execute, pb::Request, server::KafkaServer};
use anyhow::{bail, Result};
use prost::{DecodeError, Message};
use tauri::{App, Manager};

use crate::pb::Response;

pub fn register_handlers(app: &mut App, srv: Arc<Mutex<KafkaServer>>) {
    let srv = Arc::new(Mutex::new(KafkaServer::new()));
    let srv_clone = srv.clone();
    let window = app.get_window("main").unwrap();
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
    if let Ok(resp) = req.execute(srv) {
        match resp {
            crate::command::Resp::Sync(s) => Ok(s),
            crate::command::Resp::Stream(s) => bail!("not supported"),
        }
    } else {
        bail!("not right")
    }
}
