use std::fmt::{Debug, Formatter};

use serde::Deserialize;
use serde::Serialize;

use toml::to_string;
#[allow(unused_imports)]
use std::env;

use net_config::NetConfig;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Pcaps {
    pub directory_to_save: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Server {
    pub host_name: String,
    pub port: String,
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConnectionUrl {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MaxConnectionSize {
    pub size: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthServerAddres {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthApiKey {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub server: Server,
    pub connection_url: ConnectionUrl,
    pub max_connection_size: MaxConnectionSize,
    pub fusion_auth_server_addres: FusionAuthServerAddres,
    pub fusion_auth_api_key: FusionAuthApiKey,
    pub pcaps: Pcaps,
}
