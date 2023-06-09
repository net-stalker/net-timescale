use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use toml::to_string;
use net_config::NetConfig;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AgentGateway {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TranslatorGateway {
    pub(crate) addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FrontendGateway {
    pub(crate) ws_addr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    pub(crate) agent_gateway: AgentGateway,
    pub(crate) translator_gateway: TranslatorGateway,
    pub(crate) frontend_gateway: FrontendGateway,
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
            agent_gateway: AgentGateway { addr: "tcp://0.0.0.0:5555".to_string() },
            translator_gateway: TranslatorGateway { addr: "tcp://0.0.0.0:5567".to_string() },
            frontend_gateway: FrontendGateway { ws_addr: "tcp://0.0.0.0:5558".to_string() },
        };

        assert_eq!(config.unwrap(), expected_config);

        env::set_var("NET_AGENT_GATEWAY.ADDR", "tcp://localhost:5555");
        env::set_var("NET_TRANSLATOR_GATEWAY.ADDR", "tcp://localhost:5567");
        env::set_var("NET_FRONTEND_GATEWAY.WS_ADDR", "tcp://localhost:5558");

        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();

        let expected_config = Config {
            agent_gateway: AgentGateway {
                addr: "tcp://localhost:5555".to_string(),
            },
            translator_gateway: TranslatorGateway { addr: "tcp://localhost:5567".to_string() },
            frontend_gateway: FrontendGateway { ws_addr: "tcp://localhost:5558".to_string() },
        };

        assert_eq!(config.unwrap(), expected_config);
    }
}
