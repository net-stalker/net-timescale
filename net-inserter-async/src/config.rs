use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;
use net_config::NetConfig;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConnectionUrl {
    pub(crate) url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MaxConnectionSize {
    pub(crate) size: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ServerAddress {
    pub(crate) addr: String,
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
    pub(crate) connection_url: ConnectionUrl,
    pub(crate) max_connection_size: MaxConnectionSize,
    pub(crate) server_address: ServerAddress,
    pub(crate) fusion_auth_server_addres: FusionAuthServerAddres,
    pub(crate) fusion_auth_api_key: FusionAuthApiKey,
}
