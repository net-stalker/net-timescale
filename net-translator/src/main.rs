use log::info;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_translator::component::translator::Translator;

fn main() {
    init_log();
    info!("Run module");

    let pool = ThreadPool::with_name("worker".into(), 5);

    Translator::new(pool.clone()).run();

    pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
