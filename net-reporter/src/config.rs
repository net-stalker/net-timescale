use std::fmt::Debug;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use toml::to_string;

use net_config::NetConfig;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ServerAddress {
    pub(crate) address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConnectionUrl {
    pub(crate) url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MaxConnectionSize {
    pub(crate) size: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthServerAddres {
    pub(crate) addr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FusionAuthApiKey {
    pub(crate) key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) server_address: ServerAddress,
    pub(crate) connection_url: ConnectionUrl,
    pub(crate) max_connection_size: MaxConnectionSize,
    pub(crate) fusion_auth_server_addres: FusionAuthServerAddres,
    pub(crate) fusion_auth_api_key: FusionAuthApiKey,
}
