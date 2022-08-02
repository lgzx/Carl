use std::{
    collections::HashMap,
    io::Write,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use anyhow::{bail, Result};
use tauri::async_runtime;

use crate::{
    kafka::{Kafka, KafkaTuber},
    pb::{
        request::RequestCmd, AddConfig, CheckBroker, CloseChannel, Config, ListConfig, ListTopics,
        PullMessage, Request, Response,
    },
    server::Server,
};

pub enum Resp {
    Sync(Response),
    Stream((String, Receiver<String>)),
}

impl From<Response> for Resp {
    fn from(s: Response) -> Self {
        Self::Sync(s)
    }
}

pub trait Execute {
    fn execute(&self, server: &mut impl Server) -> Result<Resp>;
}

pub trait CmdExecute {
    fn execute(&self, server: &mut impl Server) -> Result<Response>;
}

pub trait ExecuteStream {
    /// return channel id and sender, when close channel , close sender in server
    fn poll(
        &self,
        server: &mut impl Server,
    ) -> Result<(String, Sender<String>, Receiver<String>, Sender<()>)>;
}

impl Execute for Request {
    fn execute(&self, server: &mut impl Server) -> Result<Resp> {
        match &self.request_cmd {
            Some(RequestCmd::Listconfig(param)) => param.execute(server).map(|r| r.into()),
            Some(RequestCmd::Addconfig(param)) => param.execute(server).map(|r| r.into()),
            Some(RequestCmd::Checkbroker(param)) => param.execute(server).map(|r| r.into()),
            Some(RequestCmd::Listtopics(param)) => param.execute(server).map(|r| r.into()),
            Some(RequestCmd::Closechannel(param)) => param.execute(server).map(|r| r.into()),
            Some(RequestCmd::Pullmessage(param)) => {
                if let Ok((r, tx, rx, closeTx)) = param.poll(server) {
                    // add sender to server so that we can close it when needed
                    server.add_sender(r.clone(), closeTx.clone()).unwrap();
                    Ok(Resp::Stream((r, rx)))
                } else {
                    Ok(Resp::Sync("ok".to_string().into()))
                }
            }
            None => todo!(),
        }
    }
}

impl CmdExecute for AddConfig {
    fn execute(&self, srv: &mut impl Server) -> Result<Response> {
        if let Some(config) = &self.cfg {
            srv.add_config(&config.broker[..], &config.topic[..])
                .map(|b| b.into())
        } else {
            Ok(false.into())
        }
    }
}
impl CmdExecute for ListConfig {
    fn execute(&self, server: &mut impl Server) -> Result<Response> {
        server.list_config().map(|v| v.into())
    }
}

impl ExecuteStream for PullMessage {
    fn poll(
        &self,
        server: &mut impl Server,
    ) -> Result<(String, Sender<String>, Receiver<String>, Sender<()>)> {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let (closeTx, closeRx): (Sender<()>, Receiver<()>) = mpsc::channel();

        let config = self.cfg.as_ref().unwrap();

        let broker = config.broker.to_string();
        let topic = config.topic.to_string();
        let topic_res = topic.clone();
        let channel = format!("{}-{}", broker.clone(), topic.clone());

        let clond_config = config.clone();
        let mut k = Kafka::new(clond_config);

        let sender = tx.clone();
        let mut writer = KafkaWriter {
            inner: sender,
            close: closeRx,
        };

        thread::spawn(move || k.consume(topic, &mut writer));

        let sig = base64::encode(channel);
        let sigvalid = String::from_iter(sig.chars().filter(|c| c.is_alphabetic()));

        Ok((sigvalid, tx, rx, closeTx))
    }
}

struct KafkaWriter {
    inner: Sender<String>,
    close: Receiver<()>,
}

impl Closeable for KafkaWriter {
    fn close_sig(&self) -> Result<()> {
        match self.close.try_recv() {
            Ok(_) => Ok(()),
            Err(e) => match e {
                mpsc::TryRecvError::Empty => bail!("empty not stoped"),
                mpsc::TryRecvError::Disconnected => Ok(()),
            },
        }
        //if let Ok(_) = self.close.recv() {
        //    Ok(())
        //} else {
        //    bail!("not stop")
        //}
    }
}

pub trait Closeable {
    fn close_sig(&self) -> Result<()>;
}

impl Write for KafkaWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let message = String::from_utf8(Vec::from(buf.clone())).unwrap();
        if let Err(_) = self.inner.send(message) {
            std::io::Result::Ok(0)
        } else {
            std::io::Result::Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::Result::Ok(())
    }
}

impl CmdExecute for CheckBroker {
    fn execute(&self, server: &mut impl Server) -> Result<Response> {
        server
            .check_broker(&self.broker)
            .map(|v| v.to_string().into())
    }
}

impl CmdExecute for ListTopics {
    fn execute(&self, server: &mut impl Server) -> Result<Response> {
        server.list_topics(&self.broker).map(|res| res.into())
    }
}

impl CmdExecute for CloseChannel {
    fn execute(&self, server: &mut impl Server) -> Result<Response> {
        server.unsubscribe(self.channel.clone()).map(|r| r.into())
    }
}
