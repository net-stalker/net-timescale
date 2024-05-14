use net_component::component::component_core::Component;
use net_reporter::config::Config;
use net_reporter::component::reporter_component::ReporterComponent;

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
        // update ip address, now we can just bind everything as before
        config.server.addr = format!("{}:{}", host_core::get_addr_for_host(&config.server.host_name).await, config.server.port);
        config
    };
    log::debug!("server ip adddress: {:?}", config.server.addr);
    let reporter_component = ReporterComponent::new(&config).await;
    
    log::info!("Created component");
    
    match reporter_component.run().await {
        Ok(_) => (),
        Err(err) => log::error!("Something went wrong during starting the component: {}", err),
    }
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
