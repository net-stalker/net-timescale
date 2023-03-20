use std::env;
use std::fmt::{Display, Formatter};
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
    pub fn new(profile_path: Option<String>) -> Result<Self, ConfigError> {
        match Self::create_config(profile_path) {
            Ok(config) => config.try_deserialize(),
            Err(e) => { Err(e) }
        }
    }

    #[cfg(debug_assertions)]
    fn create_config(config_dir_path: Option<String>) -> Result<config::Config, ConfigError> {
        let config_files = match config_dir_path {
            None => {
                Files::find_files(
                    &PathBuf::new()
                        .join(PKG_NAME)
                        .join(CONFIG_DIR),
                    "toml")
            }
            Some(path) => {
                Files::find_files(&PathBuf::from(&path), "toml")
            }
        };
        debug!("found config files {:?}", config_files);

        let mut builder = config::Config::builder();
        for i in 0..config_files.len() {
            let path_buf = config_files.get(i).unwrap().deref();
            builder = builder.add_source(File::from(path_buf));
        }

        match builder.build() {
            Ok(config) => { Ok(config) }
            Err(e) => { Err(e) }
        }
    }

    #[cfg(not(debug_assertions))]
    fn create_config(config_dir_path: Option<String>) -> Result<config::Config, ConfigError> {
        let app_config_dir_env = "NET_CONFIG_DIR";

        let config_files = match config_dir_path {
            None => {
                match env::var(app_config_dir_env) {
                    Ok(path) => {
                        Files::find_files(&PathBuf::from(&path), "toml")
                    }
                    Err(_) => {
                        let base_dir = Self::get_base_dir().unwrap();
                        let default_file_path = base_dir.home_dir()
                            .join(CONFIG_DIR)
                            .join(PKG_NAME);

                        Files::find_files(&default_file_path, "toml")
                    }
                }
            }
            Some(path) => {
                Files::find_files(&PathBuf::from(&path), "toml")
            }
        };
        debug!("found config files {:?}", config_files);

        let mut builder = config::Config::builder();
        for i in 0..config_files.len() {
            let path_buf = config_files.get(i).unwrap().deref();
            builder = builder.add_source(File::from(path_buf));
        }

        match builder.build() {
            Ok(config) => { Ok(config) }
            Err(e) => { Err(e) }
        }
    }

    #[cfg(not(debug_assertions))]
    fn get_base_dir() -> Option<directories::BaseDirs> {
        directories::BaseDirs::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_load_config() {
        env_logger::init();
        let config = Config::new(Some(".config".to_string())).unwrap();

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

        assert_eq!(config, expected_config)
    }
}
