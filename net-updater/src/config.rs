use std::fmt::Debug;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use toml::to_string;
#[allow(unused_imports)]
use std::env;

use net_config::NetConfig;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Server {
    pub host_name: String,
    pub port: String,
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TimescaleDBConnectionUrl {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TimescaleDbBufferConnectionUrl {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MaxConnectionSize {
    pub size: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub server: Server,
    pub timescaledb_connection_url: TimescaleDBConnectionUrl,
    pub timescaledb_buffer_connection_url: TimescaleDbBufferConnectionUrl,
    pub max_connection_size: MaxConnectionSize,
}
