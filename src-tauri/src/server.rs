use core::str;
use std::sync::Arc;

use anyhow::Result;

use crate::{
    command::Execute,
    config::{ConfigStore, KafkaConfig, TomlStore},
    pb::{Request, Response},
};

pub trait Server {
    fn execute(&mut self, request: Request) -> Result<Response>;

    fn add_config(&mut self, broker: &str, topic: &str) -> Result<bool>;
    fn list_config(&self) -> Result<KafkaConfig>;
}

pub struct KafkaServer {
    pub config: Box<dyn ConfigStore>,
}

unsafe impl Send for KafkaServer {}

impl Server for KafkaServer {
    fn execute(&mut self, request: Request) -> Result<Response> {
        todo!()
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
