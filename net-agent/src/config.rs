use std::fmt::{Debug, Display, Formatter};

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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Capture {
    device_name: String,
    number_packages: i32,
    buffer_size: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, NetConfig)]
pub struct Config {
    connector: Connector,
    capture: Vec<Capture>,
    log: Log,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_load_config() {
        env_logger::init();
        let config = Config::builder()
            .with_config_dir(".config".to_string())
            .build();

        let expected_config = Config {
            connector: Connector {
                endpoint: "tcp://localhost:5555".to_string(),
            },
            capture: vec![Capture {
                device_name: "en0".to_string(),
                number_packages: -1,
                buffer_size: 100,
            }],
            log: Log {
                level: LogLevel::Debug,
            },
        };

        assert_eq!(config.unwrap(), expected_config)
    }
}
