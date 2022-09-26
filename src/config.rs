use std::fmt;
use std::path::PathBuf;

use derivative::Derivative;
use directories::ProjectDirs;
use hocon::{Error, HoconLoader};
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Derivative)]
#[derive(Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Dealer {
    #[allow(dead_code)]
    #[derivative(Default(value = "true"))]
    pub enable: bool,
    #[allow(dead_code)]
    #[derivative(Default(value = "\"tcp://0.0.0.0:5555\".to_string()"))]
    pub endpoint: String,
}

#[derive(Derivative)]
#[derive(Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Data {
    #[allow(dead_code)]
    #[derivative(Default(value = "[\"any\".to_string()].to_vec()"))]
    pub devices: Vec<String>,
    #[allow(dead_code)]
    #[derivative(Default(value = "-1"))]
    pub number_packages: i32,
    #[allow(dead_code)]
    #[derivative(Default(value = "1000"))]
    pub buffer_size: i32,
}

#[derive(Derivative)]
#[derive(Serialize, Deserialize, Debug)]
#[derivative(Default)]
pub struct Config {
    #[allow(dead_code)]
    pub dealer: Dealer,
    #[allow(dead_code)]
    data: Data,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration\n {}", serde_json::to_string_pretty(&self).unwrap())
    }
}

pub struct FileLoader;

impl FileLoader {
    fn get_project_dirs() -> Option<ProjectDirs> {
        let project_dirs = ProjectDirs::from(
            "io",
            "net-stalker",
            "net-monitor");
        dbg!(project_dirs.clone());

        project_dirs
    }

    fn get_config_dir() -> PathBuf {
        let project_dirs = Self::get_project_dirs();
        match project_dirs {
            None => { panic!("not found directories") }
            Some(project_dirs) => {
                let config_dir = project_dirs.config_dir();
                dbg!(config_dir.clone());

                PathBuf::from(config_dir)
            }
        }
    }
}

#[cfg_attr(test, automock)]
pub trait FileLoaderSpec {
    fn get_config_file(&self) -> PathBuf;
}

impl FileLoaderSpec for FileLoader {
    fn get_config_file(&self) -> PathBuf {
        let config_dir = Self::get_config_dir();
        let config_file = config_dir.join("application.conf");
        dbg!(config_file.clone());

        config_file
    }
}

pub struct ConfigManager {
    pub file_loader: Box<dyn FileLoaderSpec>,
}

impl ConfigManager {
    fn get_default_config() -> Config {
        Config::default()
    }
}

pub trait ConfigSpec {
    fn load(&self) -> Config;
}

impl ConfigSpec for ConfigManager {
    fn load(&self) -> Config {
        let config_file = self.file_loader.get_config_file();

        let hocon_config: Result<HoconLoader, Error> =
            HoconLoader::new()
                .load_file(config_file);

        match hocon_config {
            Ok(loader) => {
                match loader.resolve() {
                    Ok(config) => {
                        println!("loaded configuration {:?}", config);

                        config
                    }
                    Err(error) => {
                        dbg!(error);
                        let default_config = Self::get_default_config();
                        println!("loaded default {}", default_config);

                        default_config
                    }
                }
            }
            Err(error) => {
                dbg!(error);
                let default_config = Self::get_default_config();
                println!("loaded default {}", default_config);

                default_config
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hocon::HoconLoader;

    use super::*;

    #[test]
    fn expect_load_default_configuration() {
        let config = Config::default();
        println!("{}", config);

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.endpoint, "tcp://0.0.0.0:5555");
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
        assert_eq!(config.dealer.endpoint, "tcp://0.0.0.0:4444");
        assert_eq!(config.data.devices, vec!["eth0"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 100);
    }

    #[test]
    fn expect_not_find_config_and_load_default() {
        let config = ConfigManager { file_loader: Box::new(FileLoader) as Box<dyn FileLoaderSpec> }.load();

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.endpoint, "tcp://0.0.0.0:5555");
        assert_eq!(config.data.devices, vec!["any"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 1000);
    }

    #[test]
    fn expect_find_config_and_load() {
        let mut mock = MockFileLoaderSpec::new();
        mock.expect_get_config_file()
            .return_const(".config/application.conf");

        let config = ConfigManager { file_loader: Box::new(mock) as Box<dyn FileLoaderSpec> }.load();

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.endpoint, "tcp://0.0.0.0:4444");
        assert_eq!(config.data.devices, vec!["eth0"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 100);
    }
}
