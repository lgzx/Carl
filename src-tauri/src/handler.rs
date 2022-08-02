use std::{sync::Arc, thread};

use crate::{
    command::Execute,
    pb::{Request, Response},
    server::{KafkaServer, Server, ServerState},
};
use anyhow::bail;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn run_command(
    app: AppHandle,
    state: State<'_, ServerState>,
    request: String,
) -> Result<Response, ()> {
    let req: Request = request.into();
    println!("{:?}", req);
    let mut srv = state.inner().state.clone();
    req.execute(&mut srv)
        .map(|res| match res {
            crate::command::Resp::Sync(s) => s,
            crate::command::Resp::Stream((c, rx)) => {
                let ch = c.clone();
                thread::spawn(move || {
                    while let Ok(msg) = rx.recv() {
                        //println!("send : {}", msg.clone());
                        app.clone().emit_all(&c, msg);
                    }
                });
                ch.into()
            }
        })
        .map_err(|e| ())
}
