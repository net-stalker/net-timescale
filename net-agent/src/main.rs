use log::{info};
use threadpool::ThreadPool;

use net_agent::component::capture::Capture;
use net_agent::config::Config;
use net_core::layer::NetComponent;

fn main() {
    init_log();
    info!("run module");

    let config = Config::builder().build().expect("read config error");
    let pool = ThreadPool::with_name("worker".into(), 2);

    Capture::new(pool.clone(), config).run();

    pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
