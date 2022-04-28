use std::{
    collections::HashMap,
    fmt::Debug,
    sync::atomic::{AtomicI32, Ordering},
};

pub struct KafkaConfig {
    pub brokers: String,
    pub topics: String,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: Default::default(),
            topics: Default::default(),
        }
    }
}

pub enum ConfigEnum {
    Kafka(KafkaConfig),
}

impl Debug for ConfigEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigEnum::Kafka(v) => write!(f, "{} => {}", v.brokers, v.topics),
        }
    }
}

//管理数据配置
pub trait ConfigStorage {
    fn new() -> Self;

    fn add_config(&mut self, config: ConfigEnum) -> bool;

    fn list_config(&self) -> &HashMap<i32, ConfigEnum>;

    fn delete_config(&self, config_id: i32) -> bool;
}

#[derive(Default)]
pub struct KafkaMemStorage {
    maps: HashMap<i32, ConfigEnum>,
    counter: AtomicI32,
}

impl ConfigStorage for KafkaMemStorage {
    fn new() -> Self {
        KafkaMemStorage {
            maps: HashMap::new(),
            counter: AtomicI32::new(0),
        }
    }

    fn add_config(&mut self, config: ConfigEnum) -> bool {
        let id = self.counter.fetch_add(1, Ordering::AcqRel);
        self.maps.insert(id, config);
        true
    }

    fn list_config(&self) -> &HashMap<i32, ConfigEnum> {
        &self.maps
    }

    fn delete_config(&self, config_id: i32) -> bool {
        todo!()
    }
}

pub struct KafkaStorage {}

impl Default for KafkaStorage {
    fn default() -> Self {
        Self {}
    }
}

impl ConfigStorage for KafkaStorage {
    fn add_config(&mut self, config: ConfigEnum) -> bool {
        todo!()
    }

    fn list_config(&self) -> &HashMap<i32, ConfigEnum> {
        todo!()
    }

    fn delete_config(&self, config_id: i32) -> bool {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }
}

//管理用户配置
pub trait UserSetting {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_kafka_config() {
        let mut store = KafkaMemStorage::new();
        let kafka_config = KafkaConfig::default();
        store.add_config(ConfigEnum::Kafka(kafka_config));
        let lists = store.list_config();
        lists.iter().enumerate().for_each(|f| println!("{:?}", f))
    }
}
