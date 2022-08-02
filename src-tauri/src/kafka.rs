use std::{borrow::BorrowMut, io, time::Duration};

use anyhow::Result;
use kafka::{
    client::{FetchOffset, KafkaClient},
    consumer::{Consumer, MessageSet},
    producer::{Producer, Record},
};
use prost::bytes::Buf;

use crate::{command::Closeable, pb::Config};

pub trait KafkaTuber {
    fn consume(&self, topic: String, f: &mut (impl io::Write + Closeable)) -> Result<u32>;
    fn produce(&self, topic: String, payload: String);
    fn check(&mut self) -> Result<bool>;
    fn topics(&mut self) -> Result<Vec<String>>;
}

#[derive(Debug)]
pub struct Kafka {
    client: KafkaClient,
}

impl Kafka {
    pub fn new(config: Config) -> Self {
        let broker = config.broker.to_string();
        let client = KafkaClient::new(vec![broker]);
        Self { client }
    }
}

impl KafkaTuber for Kafka {
    fn consume(&self, topic: String, f: &mut (impl io::Write + Closeable)) -> Result<u32> {
        let mut client = KafkaClient::new(self.client.hosts().to_vec());
        client.load_metadata_all().unwrap();
        let mut consumer = Consumer::from_client(client)
            .with_topic(topic)
            .with_fallback_offset(kafka::client::FetchOffset::Earliest)
            .create()
            .unwrap();
        let mut stop = false;
        'outer: loop {
            for ms in consumer.poll().unwrap().iter() {
                if let Ok(_) = f.close_sig() {
                    println!("break outer");
                    break 'outer;
                } else {
                    println!("not stop");
                };
                for m in ms.messages() {
                    let data = m.value;
                    f.write(data);
                }
                consumer.consume_messageset(ms);
            }
        }
        println!("close channel");
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
        let record = Record::from_value(&topic, payload.as_bytes());
        p.send(&record).unwrap();
    }

    fn check(&mut self) -> Result<bool> {
        if let Ok(_) = self.client.load_metadata_all() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn topics(&mut self) -> Result<Vec<String>> {
        if let Ok(_) = self.client.load_metadata_all() {
            let v = self
                .client
                .topics()
                .iter()
                .map(|t| t.name().to_string())
                .collect();
            Ok(v)
        } else {
            Ok(vec![])
        }
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
        let mut kafka = Kafka::new(Config {
            broker: "152.136.112.21:31092".to_string(),
            topic: "sample.topic".to_string(),
        });

        let mut mem_writer = Box::new(MemWriter {});
        kafka.consume("sample.topic".to_string(), &mut mem_writer);
    }

    #[test]
    fn test_tuber_can_produce() {
        let mut kafka = Kafka::new(Config {
            broker: "152.136.112.21:31092".to_string(),
            topic: "sample.topic1111".to_string(),
        });

        kafka.produce(
            "sample.topic.11111".to_string(),
            kafka
                .client
                .topics()
                .into_iter()
                .map(|x| String::from(x.name()))
                .collect(),
        );
    }
}
