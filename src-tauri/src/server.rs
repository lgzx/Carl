use core::str;

use anyhow::Result;

use crate::{
    command::Execute,
    config::{ConfigStore, TomlStore},
    pb::{cmd::RequestCmd, Cmd, Request, Response},
};

pub trait Server {
    fn execute(&self, request: Request) -> Result<Response>;

    fn add_config(&self, broker: &str, topic: &str) -> Result<bool>;
}

pub struct KafkaServer {
    pub config: Box<dyn ConfigStore>,
}

impl Server for KafkaServer {
    fn execute(&self, request: Request) -> Result<Response> {
        let cmd: Cmd = request.into();

        match cmd.request_cmd {
            Some(RequestCmd::Addconfig(cmd)) => cmd.execute(self),
            None => todo!(),
        }
    }

    fn add_config(&self, config_name: &str, config: &str) -> Result<bool> {
        todo!();
    }
}

impl KafkaServer {
    pub fn new() -> Self {
        Self {
            config: Box::new(TomlStore::new()),
        }
    }
}
