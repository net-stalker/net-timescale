use std::net::IpAddr;
use std::path::Path;

use net_reporter::config::Config;
use net_reporter::component::reporter::Reporter;
use tokio::fs;

async fn get_addr_for_host(config: &Config) -> String {
    let host_name = config.server.host_name.as_str();
    let hosts_file = Path::new("/etc/hosts");
    let contents = fs::read_to_string(hosts_file).await.expect("Failed to read /etc/hosts");

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(ip_addr) = parts[0].parse::<IpAddr>() {
                if parts[1] == host_name {
                    return format!("{}:{}", ip_addr, config.server.port);
                }
            }
        }
    }
    panic!("Failed to find ip address for host name: {}", host_name);
}

#[tokio::main]
async fn main() {
    init_log();
    log::info!("Run module");
    let config = if cfg!(debug_assertions) {
        println!("Running in debug mode");
        Config::builder().build().expect("read config error")
    } else {
        println!("Running in release mode");
        let config_path = std::env::var("CONFIG_PATH").unwrap();
        let mut config = Config::new(&config_path).build().expect("read config error");
        // update ip address, now we can just bind everything as before
        config.server.addr = get_addr_for_host(&config).await;
        config
    };
    log::debug!("server ip adddress: {:?}", config.server.addr);
    let reporter_component = Reporter::new(config).await;
    
    log::info!("Created component");
    
    reporter_component.run().await;
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
