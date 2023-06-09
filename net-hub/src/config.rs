use std::fmt::{Debug, Formatter};

use log::Level;
use serde::{Deserialize, Serialize};
use toml::to_string;

use net_config::NetConfig;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Connector {
    endpoint: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Into<Level> for LogLevel {
    fn into(self) -> Level {
        match self {
            LogLevel::Trace => Level::Trace,
            LogLevel::Debug => Level::Debug,
            LogLevel::Info => Level::Info,
            LogLevel::Warn => Level::Warn,
            LogLevel::Error => Level::Error,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Log {
    level: LogLevel,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    connector: Connector,
    log: Log,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_load_config() {
        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();

        let expected_config = Config {
            connector: Connector {
                endpoint: "tcp://api.hub.netstalker.io:5555".to_string(),
            },
            log: Log {
                level: LogLevel::Info,
            },
        };

        assert_eq!(config.unwrap(), expected_config)
    }
}
