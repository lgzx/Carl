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
    #[prost(string, tag="1")]
    pub route: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub data: ::prost::alloc::string::String,
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
pub struct Cmd {
    #[prost(oneof="cmd::RequestCmd", tags="1, 2")]
    pub request_cmd: ::core::option::Option<cmd::RequestCmd>,
}
/// Nested message and enum types in `Cmd`.
pub mod cmd {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestCmd {
        #[prost(message, tag="1")]
        Addconfig(super::AddConfig),
        #[prost(message, tag="2")]
        Listconfig(super::ListConfig),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddConfig {
    #[prost(message, optional, tag="1")]
    pub cfg: ::core::option::Option<Config>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfig {
}
