use std::fmt::{Debug, Display, Formatter};

use log::Level;
use serde::{Deserialize, Serialize};
use toml::to_string;

use net_config::NetConfig;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AgentConnector {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Capture {
    pub(crate) device_name: String,
    pub(crate) number_packages: i32,
    pub(crate) buffer_size: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) agent_connector: AgentConnector,
    pub(crate) capture: Capture,
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
            agent_connector: AgentConnector {
                addr: "tcp://0.0.0.0:5555".to_string(),
            },
            capture: Capture {
                device_name: "en0".to_string(),
                number_packages: -1,
                buffer_size: 1000,
            },
        };

        assert_eq!(config.unwrap(), expected_config);

        env::set_var("NET_AGENT_CONNECTOR.ADDR", "tcp://localhost:5557");
        env::set_var("NET_CAPTURE.DEVICE_NAME", "en1");
        env::set_var("NET_CAPTURE.NUMBER_PACKAGES", "1");
        env::set_var("NET_CAPTURE.buffer_size", "10");

        let config = Config::builder().build();

        let expected_config = Config {
            agent_connector: AgentConnector {
                addr: "tcp://localhost:5557".to_string(),
            },
            capture: Capture {
                device_name: "en1".to_string(),
                number_packages: 1,
                buffer_size: 10,
            },
        };

        assert_eq!(config.unwrap(), expected_config);
    }
}
