use net_reporter::config::Config;
use net_reporter::component::reporter::Reporter;

#[tokio::main]
async fn main() {
    init_log();
    log::info!("Run module");
    
    let config = Config::builder().build().expect("read config error");

    let reporter_component = Reporter::new(config).await;
    
    log::info!("Created component");
    
    reporter_component.run().await;
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}