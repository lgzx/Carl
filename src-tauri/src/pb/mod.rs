pub mod abi;

use core::panic;
use std::{
    collections::{BTreeMap, HashMap},
    io::Cursor,
};

pub use abi::*;
use prost::{DecodeError, Message};
use serde_json::Value;

use crate::config::KafkaConfig;

impl From<&str> for Config {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

impl From<String> for Request {
    fn from(s: String) -> Self {
        let byte_info: Vec<u8> = s.split(",").map(|sstr| sstr.to_string().parse::<u8>().unwrap()).into_iter().collect();
        let req: Result<Request, DecodeError> = Message::decode(&mut Cursor::new(&byte_info));
        if let Ok(info) = req {
            info
        } else {
            Self {
                ..Default::default()
            }
        }
    }
}

impl From<bool> for Response {
    fn from(b: bool) -> Self {
        let mut res = Self {
            ..Default::default()
        };
        match b {
            true => res.status = "ok".to_string(),
            false => res.status = "error".to_string(),
        };
        res
    }
}

impl From<KafkaConfig> for Response {
    fn from(config: KafkaConfig) -> Self {
        Self {
            data: serde_json::to_string(&config.clusters).unwrap(),
            status: "ok".to_string(),
        }
    }
}

impl From<String> for Response {
    fn from(s: String) -> Self {
        Self {
            status: "ok".to_string(),
            data: s,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
