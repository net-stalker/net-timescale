use log::info;
use threadpool::ThreadPool;

use net_agent::component::capture::Capture;
use net_core::layer::NetComponent;
use net_hub::component::hub::Hub;
use net_hub::config::Config;
use net_timescale::component::timescale::Timescale;
use net_translator::component::translator::Translator;

fn main() {
    init_log();
    info!("Run module");

    let thread_pool = ThreadPool::with_name("worker".into(), 20);

    //FIXME Currently OCP is violated. The modules should be scanned based on dependencies, iterate through it and start it dynamically
    Capture::new(thread_pool.clone()).run();
    Hub::new(thread_pool.clone()).run();
    Translator::new(thread_pool.clone()).run();
    let manager = r2d2_postgres::PostgresConnectionManager::new(
        "postgres://postgres:PsWDgxZb@localhost".parse().unwrap(),
        postgres::NoTls,
    );
    let connection_pool = r2d2::Pool::builder().max_size(10).build(manager).unwrap();
    Timescale::new(thread_pool.clone(), connection_pool).run();

    thread_pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
