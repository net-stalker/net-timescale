use std::{fmt, fs};

use derivative::Derivative;
use directories::ProjectDirs;
use hocon::HoconLoader;
use serde::{Deserialize, Serialize};

#[derive(Derivative)]
#[derive(Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Dealer {
    #[allow(dead_code)]
    #[derivative(Default(value = "true"))]
    enable: bool,
    #[allow(dead_code)]
    #[derivative(Default(value = "\"tcp://*:5555\".to_string()"))]
    addr: String,
}

#[derive(Derivative)]
#[derive(Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Data {
    #[allow(dead_code)]
    #[derivative(Default(value = "[\"any\".to_string()].to_vec()"))]
    devices: Vec<String>,
    #[allow(dead_code)]
    #[derivative(Default(value = "-1"))]
    number_packages: i32,
    #[allow(dead_code)]
    #[derivative(Default(value = "1000"))]
    buffer_size: i32,
}

#[derive(Derivative)]
#[derive(Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Config {
    #[allow(dead_code)]
    dealer: Dealer,
    #[allow(dead_code)]
    data: Data,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "default configuration:\n {}", serde_json::to_string_pretty(&self).unwrap())
    }
}

pub fn load_configuration() -> Config {
    let project_dirs = ProjectDirs::from(
        "io",
        "net-stalker",
        "net-monitor");
    dbg!(project_dirs.clone());

    match project_dirs {
        None => { Config::default() }
        Some(project_dirs) => {
            let config_dir = project_dirs.config_dir();
            dbg!(config_dir.clone());

            let config_file = fs::read_to_string(config_dir.join(".config/application.conf"));
            match config_file {
                Ok(file) => {
                    dbg!(file.clone());

                    let config: Result<Config, _> = HoconLoader::new()
                        .load_file(file)
                        .unwrap()
                        .resolve();

                    match config {
                        Ok(config) => {
                            println!("{:?}", config);

                            config
                        }
                        Err(error) => {
                            println!("\n{:?}", error);

                            Config::default()
                        }
                    }
                }
                Err(error) => {
                    println!("\n{}", error);

                    Config::default()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use directories::{BaseDirs, ProjectDirs};
    use hocon::{Error, HoconLoader};

    use super::*;

    #[test]
    fn expect_load_default_configuration() {
        let config = Config::default();
        println!("{}", config);

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.addr, "tcp://*:5555".to_string());
        assert_eq!(config.data.devices, vec!["any"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 1000);
    }

    #[test]
    fn expect_load_configuration_from_file() {
        let config: Result<Config, _> = HoconLoader::new()
            .load_file(".config/application.conf")
            .unwrap()
            .resolve();

        assert!(config.is_ok());
        let config = config.unwrap();
        println!("{}", config);

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.addr, "tcp://*:5555".to_string());
        assert_eq!(config.data.devices, vec!["eth0", "any"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 1000);
    }

    #[test]
    fn expect_find_default_config_directory() {
        let config = load_configuration();

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.addr, "tcp://*:5555".to_string());
        assert_eq!(config.data.devices, vec!["any"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 1000);
    }
}
