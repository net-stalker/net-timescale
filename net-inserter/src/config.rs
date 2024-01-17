use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;
use net_config::NetConfig;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConnectionUrl {
    pub(crate) url: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MaxConnectionSize {
    pub(crate) size: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct HubConnector {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) connection_url: ConnectionUrl,
    pub(crate) max_connection_size: MaxConnectionSize,
    pub(crate) hub_connector: HubConnector,
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
            connection_url: ConnectionUrl { url:
                "postgres://postgres:PsWDgxZb@localhost:5433/?sslmode=require&sslcert=net-inserter/src/.ssl/client.crt&sslkey=net-inserter/src/.ssl/client.key".to_string()
            },
            max_connection_size: MaxConnectionSize { size: "10".to_string() },
            hub_connector: HubConnector { addr: "tcp://0.0.0.0:5557".to_string() },
        };

        assert_eq!(config.unwrap(), expected_config);

        env::set_var("NET_CONNECTION_URL.URL",
                     "postgres://postgres:PsWDgxZb@localhost:5433/?sslmode=require&sslcert=net-inserter/src/.ssl/client.crt&sslkey=net-inserter/src/.ssl/client.key");
        // TODO: investigate if there a possibility to set intgeer values in set_var
        env::set_var("NET_MAX_CONNECTION_SIZE.SIZE", "10");
        env::set_var("NET_HUB_CONNECTOR.ADDR", "tcp://localhost:5557");

        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();
        let expected_config = Config {
            connection_url: ConnectionUrl { url:
                "postgres://postgres:PsWDgxZb@localhost:5433/?sslmode=require&sslcert=net-inserter/src/.ssl/client.crt&sslkey=net-inserter/src/.ssl/client.key".to_string()
            },
            max_connection_size: MaxConnectionSize { size: "10".to_string() },
            hub_connector: HubConnector { addr: "tcp://localhost:5557".to_string() },
        };
        assert_eq!(config.unwrap(), expected_config);
    }
}
