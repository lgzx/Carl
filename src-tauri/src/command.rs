use std::collections::HashMap;

use anyhow::{Ok, Result};

use crate::{
    pb::{AddConfig, Response},
    server::Server,
};

pub trait Execute {
    fn execute(&self, server: &impl Server) -> Result<Response>;
}

impl Execute for AddConfig {
    fn execute(&self, srv: &impl Server) -> Result<Response> {
        if let Some(config) = &self.cfg {
            let mut map = HashMap::new();
            map.insert("broker", &config.broker);
            map.insert("topic", &config.topic);

            let config: String = serde_json::to_string(&map).unwrap();
            srv.add_config("kafka-config", &config[..])
                .map(|b| b.into())
        } else {
            Ok(false.into())
        }
    }
}
