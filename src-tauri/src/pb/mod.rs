pub mod abi;

use core::panic;
use std::collections::HashMap;

pub use abi::*;
use serde_json::Value;

use crate::config::KafkaConfig;

impl From<&str> for Config {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

impl From<&str> for Request {
    fn from(s: &str) -> Self {
        Self {
            ..Default::default()
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
