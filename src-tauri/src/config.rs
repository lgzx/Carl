use std::{
    any, env,
    fs::{self},
};

use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Default, Deserialize, Clone, Serialize)]
pub struct KafkaConfig {
    pub clusters: Box<Vec<ClusterConfig>>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct ClusterConfig {
    broker: String,
    topic: String,
}

#[derive(Default)]
pub struct TomlStore {
    pub config_path: String,
    pub configs: KafkaConfig,
}

impl TomlStore {
    pub fn new_with_path(config_path: String) -> Self {
        Self {
            config_path: config_path.clone(),
            configs: toml::from_str(&fs::read_to_string(&config_path[..]).unwrap()).unwrap(),
            ..Default::default()
        }
    }

    pub fn new() -> Self {
        let home_path = env::home_dir().unwrap();
        let config_path = format!("{}/.carl", home_path.to_str().unwrap());
        let mut store = Self {
            config_path,
            ..Default::default()
        };

        let config: KafkaConfig =
            toml::from_str(&fs::read_to_string(&store.config_path).unwrap()).unwrap();

        store.configs = config;

        store
    }

    fn persist(&self) -> Result<bool> {
        let v = toml::to_string(&self.configs)?;
        fs::write(&self.config_path, v).map(|v| Ok(true))?
    }
}

impl ConfigStore for TomlStore {
    fn add(&mut self, k: &str, v: &str) -> Result<bool> {
        self.configs.clusters.push(ClusterConfig {
            broker: k.to_string(),
            topic: v.to_string(),
        });
        self.persist()?;
        Ok(true)
    }

    fn list(&self) -> Result<KafkaConfig> {
        Ok(self.configs.clone())
    }
}

// 配置存储
pub trait ConfigStore {
    fn add(&mut self, k: &str, v: &str) -> Result<bool>;
    fn list(&self) -> Result<KafkaConfig>;
}

#[cfg(test)]
mod tests {
    use crate::config::KafkaConfig;

    use super::{ConfigStore, TomlStore};

    fn get_config() -> TomlStore {
        TomlStore::new_with_path("/Users/zhangruobin/.carltest".to_string())
    }

    #[test]
    fn test_new_toml() {
        let config = get_config();
    }

    #[test]
    fn test_add_config() {
        let mut config = get_config();
        config.add("btoker2", "topic2");
    }

    #[test]
    fn test_list_config() {
        let config = get_config();
        let lists = config.list().unwrap();
        assert_eq!(lists.clusters.len(), 1);
    }
}
