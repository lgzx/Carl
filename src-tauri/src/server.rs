use core::str;
use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex, MutexGuard},
};

use anyhow::Result;

use crate::{
    command::Execute,
    config::{ConfigStore, KafkaConfig, TomlStore},
    kafka::{Kafka, KafkaTuber},
    pb::{request::RequestCmd, Config, Request, Response},
};

pub trait Server {
    fn add_config(&mut self, broker: &str, topic: &str) -> Result<bool>;
    fn list_config(&self) -> Result<KafkaConfig>;
    fn check_broker(&self, broker: &str) -> Result<bool>;
    fn list_topics(&self, broker: &str) -> Result<Vec<String>>;
    fn add_sender(&mut self, channel: String, sender: Sender<()>) -> Result<bool>;
    fn unsubscribe(&mut self, channel: String) -> Result<bool>;
}

pub struct KafkaServer {
    pub config: Box<dyn ConfigStore>,
    pub senders: HashMap<String, Sender<()>>,
}

pub struct ServerState {
    pub state: Arc<Mutex<KafkaServer>>,
}

unsafe impl Send for KafkaServer {}

impl Server for Arc<Mutex<KafkaServer>> {
    fn add_config(&mut self, broker: &str, topic: &str) -> Result<bool> {
        self.lock().unwrap().add_config(broker, topic)
    }

    fn list_config(&self) -> Result<KafkaConfig> {
        self.lock().unwrap().list_config()
    }

    fn check_broker(&self, broker: &str) -> Result<bool> {
        self.lock().unwrap().check_broker(broker)
    }

    fn list_topics(&self, broker: &str) -> Result<Vec<String>> {
        self.lock().unwrap().list_topics(broker)
    }

    fn add_sender(&mut self, channel: String, sender: Sender<()>) -> Result<bool> {
        self.lock().unwrap().add_sender(channel, sender)
    }

    fn unsubscribe(&mut self, channel: String) -> Result<bool> {
        self.lock().unwrap().unsubscribe(channel)
    }
}

impl Server for KafkaServer {
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

    fn list_topics(&self, broker: &str) -> Result<Vec<String>> {
        let mut k = Kafka::new(Config {
            broker: broker.to_string(),
            ..Default::default()
        });

        k.topics()
    }

    fn add_sender(&mut self, channel: String, sender: Sender<()>) -> Result<bool> {
        println!("add sender {} {:?}", channel, &sender);
        self.senders.insert(channel, sender);
        Ok(true)
    }

    fn unsubscribe(&mut self, channel: String) -> Result<bool> {
        if let Some(s) = self.senders.get(&channel) {
            if let Ok(_) = s.send(()) {
                println!("send stop sig success");
            }
            self.senders.remove(&channel);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl KafkaServer {
    pub fn new() -> Self {
        Self {
            config: Box::new(TomlStore::new()),
            senders: HashMap::new(),
        }
    }
}
