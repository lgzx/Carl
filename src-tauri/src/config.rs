use crate::search::SearchPattern;

pub struct Kafka {
    brokers: String,
    topics: String,
    filter: SearchPattern,
}
