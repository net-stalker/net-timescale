use std::env;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::path::PathBuf;

use config::{ConfigError, File};
use log::{debug, Level};
use serde::{Deserialize, Serialize};
use toml::to_string;

use net_core::file::files::Files;

const CONFIG_DIR: &str = ".config";

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    connector: Connector,
    capture: Vec<Capture>,
    log: Log,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(self).unwrap())
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Debug)]
pub struct ConfigBuilder {
    config_path: PathBuf,
}

#[cfg(debug_assertions)]
impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder { config_path: PathBuf::new().join(PKG_NAME).join(CONFIG_DIR) }
    }
}

#[cfg(not(debug_assertions))]
impl Default for ConfigBuilder {
    fn default() -> Self {
        if env::var("NET_CONFIG_DIR").is_ok() {
            return ConfigBuilder { config_path: PathBuf::from(&env::var("NET_CONFIG_DIR").unwrap()) };
        }

        let base_dir = Self::get_base_dir().unwrap();
        ConfigBuilder { config_path: PathBuf::from(base_dir.home_dir()).join(CONFIG_DIR).join(PKG_NAME) }
    }
}

impl ConfigBuilder {
    pub(crate) fn with_config_dir(mut self, config_path: String) -> Self {
        self.config_path = PathBuf::from(config_path);
        self
    }

    #[cfg(not(debug_assertions))]
    fn get_base_dir() -> Option<directories::BaseDirs> {
        directories::BaseDirs::new()
    }
}

impl<'de> ConfigBuilder {
    pub fn build(&self) -> Result<Config, ConfigError> {
        debug!("{:?}", self);

        let config_files = Files::find_files(&self.config_path, "toml");
        debug!("found config files {:?}", config_files);

        match Self::create_config(config_files) {
            Ok(config) => { config.try_deserialize::<'de, Config>() }
            Err(e) => { Err(e) }
        }
    }

    fn create_config(config_files: Vec<PathBuf>) -> Result<config::Config, ConfigError> {
        let mut builder = config::Config::builder();

        for i in 0..config_files.len() {
            let path_buf = config_files.get(i).unwrap().deref();
            builder = builder.add_source(File::from(path_buf));
        }

        builder.build()
    }
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
                device_name: "eth0".to_string(),
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
