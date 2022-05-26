use std::{borrow::BorrowMut, io, time::Duration};

use anyhow::Result;
use kafka::{
    client::KafkaClient,
    consumer::Consumer,
    producer::{Producer, Record},
};
use prost::bytes::Buf;

use crate::pb::Config;

pub trait KafkaTuber {
    fn consume(&self, topic: String, f: &mut impl io::Write) -> Result<u32>;
    fn produce(&self, topic: String, payload: String);
}

#[derive(Debug)]
pub struct Kafka {
    pub client: KafkaClient,
}

impl Kafka {
    pub fn new(config: &Config) -> Self {
        let broker = config.broker.to_string();
        let mut client = KafkaClient::new(vec![broker]);
        client.load_metadata_all().unwrap();
        Self { client }
    }
}

impl KafkaTuber for Kafka {
    fn consume(&self, topic: String, f: &mut impl io::Write) -> Result<u32> {
        let mut client = KafkaClient::new(self.client.hosts().to_vec());
        client.load_metadata_all().unwrap();
        let mut consumer = Consumer::from_client(client)
            .with_topic(topic)
            .create()
            .unwrap();
        loop {
            for ms in consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    let data = m.value;
                    f.write(data);
                }
                consumer.consume_messageset(ms);
            }
        }
        Ok(5)
    }

    fn produce(&self, topic: String, payload: String) {
        let mut client = KafkaClient::new(self.client.hosts().to_vec());
        client.load_metadata_all().unwrap();
        let mut p = Producer::from_client(client)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(kafka::client::RequiredAcks::One)
            .create()
            .unwrap();
        let record = Record::from_value("sample.topic", payload.as_bytes());
        p.send(&record).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, str};

    use super::*;

    struct MemWriter {}

    impl Write for MemWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            println!("okkkkkk : {:?}", str::from_utf8(buf).unwrap());
            Ok(1)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_tuber_can_consume() {
        let mut kafka = Kafka::new(&Config {
            broker: "152.136.112.21:31092".to_string(),
            topic: "sample.topic".to_string(),
        });

        let mut mem_writer = Box::new(MemWriter {});
        kafka.consume("sample.topic".to_string(), &mut mem_writer);
    }

    #[test]
    fn test_tuber_can_produce() {
        let mut kafka = Kafka::new(&Config {
            broker: "152.136.112.21:31092".to_string(),
            topic: "sample.topic".to_string(),
        });

        kafka.produce("sample.topic".to_string(), "test".to_string());
    }
}
