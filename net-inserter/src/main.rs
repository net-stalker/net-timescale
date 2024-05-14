use net_component::component::component_core::Component;
use net_inserter::config::Config;
use net_inserter::component::inserter_component::InserterComponent;

#[tokio::main]
async fn main() {
    init_log();
    log::info!("Run module");
    let config = if cfg!(debug_assertions) {
        log::info!("Running in debug mode");
        Config::builder().build().expect("read config error")
    } else {
        log::info!("Running in release mode");
        let config_path = std::env::var("CONFIG_PATH").unwrap();
        let mut config = Config::new(&config_path).build().expect("read config error");
        config.server.addr = format!("{}:{}", host_core::get_addr_for_host(&config.server.host_name).await, &config.server.port);
        config
    };

    let inserter_component = InserterComponent::new(&config).await;

    log::info!("Created component");
    
    match inserter_component.run().await {
        Ok(_) => (),
        Err(err) => log::error!("Something went wrong during starting the component: {}", err),
    }
}

fn init_log() {     
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
