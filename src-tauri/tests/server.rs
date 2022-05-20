use anyhow::Result;
use app::{
    config::TomlStore,
    pb::{cmd::RequestCmd, AddConfig, Cmd, Config, Request},
    server::{KafkaServer, Server},
};

fn get_server() -> Box<dyn Server> {
    let mut srv = Box::new(KafkaServer::new());
    srv.config = Box::new(TomlStore::new_with_path(
        "/Users/zhangruobin/.carltest".to_string(),
    ));
    srv
}

#[test]
fn test_server_add_config() -> Result<()> {
    let mut srv = get_server();
    let req = r#"{"route":"addconfig", "data":"{\"broker\":\"broer2\", \"topic\":\"add\"}"}"#;

    let res = srv.execute(req.into())?;

    assert_eq!(res.status, "ok");

    Ok(())
}

#[test]
fn test_server_get_list() -> Result<()> {
    let mut srv = get_server();
    let req = r#"{"route":"listconfig", "data":""}"#;
    let res = &srv.execute(req.into())?;

    assert_eq!(res.data, "pl");
    Ok(())
}
