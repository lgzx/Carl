use std::collections::HashMap;

use anyhow::{Ok, Result};

use crate::{
    pb::{request::RequestCmd, AddConfig, ListConfig, PullMessage, Request, Response},
    server::Server,
};

pub trait Execute {
    fn execute(&self, server: &mut impl Server) -> Result<Response>;
}

impl Execute for Request {
    fn execute(&self, server: &mut impl Server) -> Result<Response> {
        match &self.request_cmd {
            Some(RequestCmd::Listconfig(param)) => param.execute(server),
            Some(RequestCmd::Addconfig(param)) => param.execute(server),
            Some(RequestCmd::Pullmessage(param)) => param.execute(server),
            None => todo!(),
        }
    }
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

impl Execute for PullMessage {
    fn execute(&self, server: &mut impl Server) -> Result<Response> {
        Ok("tttttt".to_string().into())
    }
}
