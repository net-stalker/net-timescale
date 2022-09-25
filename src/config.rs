use std::fmt;

use derivative::Derivative;
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

#[cfg(test)]
mod tests {
    use hocon::HoconLoader;

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
    fn expect_load_configuration() {
        let config: Result<Config, _> = HoconLoader::new()
            .load_file(".config/application.conf")
            .unwrap()
            .resolve();

        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.dealer.enable, true);
        assert_eq!(config.dealer.addr, "tcp://*:5555".to_string());
        assert_eq!(config.data.devices, vec!["eth0", "any"]);
        assert_eq!(config.data.number_packages, -1);
        assert_eq!(config.data.buffer_size, 1000);
    }
}
