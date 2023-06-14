use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;

use net_config::NetConfig;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TranslatorEndpoint {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TranslatorConnector {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) translator_endpoint: TranslatorEndpoint,
    pub(crate) translator_connector: TranslatorConnector,
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn expected_load_config() {
        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();

        let expected_config = Config {
            translator_endpoint: TranslatorEndpoint {
                addr: "tcp://0.0.0.0:5557".to_string(),
            },
            translator_connector: TranslatorConnector { addr: "tcp://0.0.0.0:5567".to_string() },
        };

        assert_eq!(config.unwrap(), expected_config);

        env::set_var("NET_TRANSLATOR_ENDPOINT.ADDR", "tcp://localhost:5557");
        env::set_var("NET_TRANSLATOR_CONNECTOR.ADDR", "tcp://localhost:5567");

        let config = Config::builder().build();

        let expected_config = Config {
            translator_endpoint: TranslatorEndpoint {
                addr: "tcp://localhost:5557".to_string(),
            },
            translator_connector: TranslatorConnector { addr: "tcp://localhost:5567".to_string() },
        };

        assert_eq!(config.unwrap(), expected_config);
    }
}
