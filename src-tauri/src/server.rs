use core::str;
use std::sync::{Arc, Mutex};

use anyhow::Result;

use crate::{
    command::Execute,
    config::{ConfigStore, KafkaConfig, TomlStore},
    kafka::{Kafka, KafkaTuber},
    pb::{request::RequestCmd, Config, Request, Response},
};

pub trait Server {
    fn execute(&mut self, request: Request) -> Result<Response>;

    fn add_config(&mut self, broker: &str, topic: &str) -> Result<bool>;
    fn list_config(&self) -> Result<KafkaConfig>;
    fn check_broker(&self, broker: &str) -> Result<bool>;
}

pub struct KafkaServer {
    pub config: Box<dyn ConfigStore>,
}

pub struct ServerState {
    pub state: Arc<Mutex<KafkaServer>>,
}

unsafe impl Send for KafkaServer {}

impl Server for KafkaServer {
    fn execute(&mut self, request: Request) -> Result<Response> {
        match request.request_cmd {
            Some(RequestCmd::Addconfig(params)) => params.execute(self),
            Some(RequestCmd::Listconfig(params)) => params.execute(self),
            Some(RequestCmd::Pullmessage(params)) => params.execute(self),
            Some(RequestCmd::Checkbroker(params)) => params.execute(self),
            None => anyhow::bail!("no request found"),
        }
    }

    fn add_config(&mut self, broker: &str, topic: &str) -> Result<bool> {
        self.config.add(broker, topic)
    }

    fn list_config(&self) -> Result<KafkaConfig> {
        self.config.list()
    }

    fn check_broker(&self, broker: &str) -> Result<bool> {
        let mut k = Kafka::new(Config {
            broker: broker.to_string(),
            topic: "".to_string(),
        });

        k.check()
    }
}

impl KafkaServer {
    pub fn new() -> Self {
        Self {
            config: Box::new(TomlStore::new()),
        }
    }
}
