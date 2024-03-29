#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(string, tag="1")]
    pub broker: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub topic: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::RequestCmd", tags="1, 2, 3, 4, 5, 6")]
    pub request_cmd: ::core::option::Option<request::RequestCmd>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestCmd {
        #[prost(message, tag="1")]
        Addconfig(super::AddConfig),
        #[prost(message, tag="2")]
        Listconfig(super::ListConfig),
        #[prost(message, tag="3")]
        Pullmessage(super::PullMessage),
        #[prost(message, tag="4")]
        Checkbroker(super::CheckBroker),
        #[prost(message, tag="5")]
        Listtopics(super::ListTopics),
        #[prost(message, tag="6")]
        Closechannel(super::CloseChannel),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub data: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddConfig {
    #[prost(message, optional, tag="1")]
    pub cfg: ::core::option::Option<Config>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBroker {
    #[prost(string, tag="1")]
    pub broker: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfig {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullMessage {
    #[prost(message, optional, tag="1")]
    pub cfg: ::core::option::Option<Config>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopics {
    #[prost(string, tag="1")]
    pub broker: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseChannel {
    #[prost(string, tag="1")]
    pub channel: ::prost::alloc::string::String,
}
