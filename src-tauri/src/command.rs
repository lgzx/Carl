use std::collections::HashMap;

use anyhow::{Ok, Result};

use crate::{
    pb::{AddConfig, ListConfig, Response},
    server::Server,
};

pub trait Execute {
    fn execute(&self, server: &mut impl Server) -> Result<Response>;
}

impl Execute for AddConfig {
    fn execute(&self, srv: &mut impl Server) -> Result<Response> {
        if let Some(config) = &self.cfg {
            srv.add_config(&config.broker[..], &config.topic[..])
                .map(|b| b.into())
        } else {
            Ok(false.into())
        }
    }
}
impl Execute for ListConfig {
    fn execute(&self, server: &mut impl Server) -> Result<Response> {
        server.list_config().map(|v| v.into())
    }
}
