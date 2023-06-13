use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;
use net_config::NetConfig;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConnectionString {
    pub(crate) url: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MaxConnectionSize {
    pub(crate) size: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TimescaleRouter {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TranslatorConnector {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) connection_string: ConnectionString,
    pub(crate) max_connection_size: MaxConnectionSize,
    pub(crate) timescale_router: TimescaleRouter,
    pub(crate) translator_connector: TranslatorConnector,
}

#[cfg(test)]
mod tests {
    use std::env;
    use toml::value::Time;
    use super::*;

    #[test]
    fn expected_load_config() {
        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();

        let expected_config = Config {
            connection_string: ConnectionString { url:
                "postgres://postgres:PsWDgxZb@localhost/?sslmode=require&sslcert=/.ssl/client.crt&sslkey=/.ssl/client.key".to_string()
            },
            max_connection_size: MaxConnectionSize { size: "10".to_string() },
            timescale_router: TimescaleRouter { addr: "tcp://0.0.0.0:5558".to_string() },
            translator_connector: TranslatorConnector { addr: "tcp://0.0.0.0:5557".to_string() },
        };

        assert_eq!(config.unwrap(), expected_config);

        env::set_var("NET_CONNECTION_STRING.URL",
                     "postgres://postgres:PsWDgxZb@localhost/?sslmode=require&sslcert=/.ssl/client.crt&sslkey=/.ssl/client.key");
        env::set_var("NET_MAX_CONNECTION_SIZE.SIZE", "10");
        env::set_var("NET_TIMESCALE_ROUTER.ADDR", "tcp://localhost:5558");
        env::set_var("NET_TRANSLATOR_CONNECTOR.ADDR", "tcp://localhost:5557");

        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();
        let expected_config = Config {
            connection_string: ConnectionString { url:
            "postgres://postgres:PsWDgxZb@localhost/?sslmode=require&sslcert=/.ssl/client.crt&sslkey=/.ssl/client.key".to_string()
            },
            max_connection_size: MaxConnectionSize { size: "10".to_string() },
            timescale_router: TimescaleRouter { addr: "tcp://localhost:5558".to_string() },
            translator_connector: TranslatorConnector { addr: "tcp://localhost:5557".to_string() },
        };
        assert_eq!(config.unwrap(), expected_config);
    }
}
