#[derive(Debug, Default, serde::Serialize, Clone)]
pub struct Payload {
    event_name: String,
    content: String,
}

impl Payload {
    pub fn new(name: &str, content: &str) -> Payload {
        Payload {
            event_name: name.to_string(),
            content: content.to_string(),
        }
    }
}
