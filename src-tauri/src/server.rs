use core::str;

use anyhow::Result;

use crate::{
    command::Execute,
    config::{ConfigStore, KafkaConfig, TomlStore},
    pb::{cmd::RequestCmd, Cmd, Request, Response},
};

pub trait Server {
    fn execute(&mut self, request: Request) -> Result<Response>;

    fn add_config(&mut self, broker: &str, topic: &str) -> Result<bool>;
    fn list_config(&self) -> Result<KafkaConfig>;
}

pub struct KafkaServer {
    pub config: Box<dyn ConfigStore>,
}

impl Server for KafkaServer {
    fn execute(&mut self, request: Request) -> Result<Response> {
        let cmd: Cmd = request.into();

        match cmd.request_cmd {
            Some(RequestCmd::Addconfig(cmd)) => cmd.execute(self),
            Some(RequestCmd::Listconfig(cmd)) => cmd.execute(self),
            None => todo!(),
        }
    }

    fn add_config(&mut self, broker: &str, topic: &str) -> Result<bool> {
        self.config.add(broker, topic)
    }

    fn list_config(&self) -> Result<KafkaConfig> {
        self.config.list()
    }
}

impl KafkaServer {
    pub fn new() -> Self {
        Self {
            config: Box::new(TomlStore::new()),
        }
    }
}
