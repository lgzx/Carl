use crate::{
    pb::{Request, Response},
    server::{Server, ServerState},
};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn run_command(
    _app: AppHandle,
    state: State<'_, ServerState>,
    request: String,
) -> Result<Response, ()> {
    let req: Request = request.into();
    println!("{:?}", req);
    state
        .inner()
        .state
        .lock()
        .unwrap()
        .execute(req)
        .map(|res| res)
        .map_err(|e| ())
}
