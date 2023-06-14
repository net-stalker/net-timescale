use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::info;
use threadpool::ThreadPool;

use net_agent::component::capture::Capture;
use net_core::layer::NetComponent;
use net_hub::component::hub::Hub;
use net_timescale::component::timescale::Timescale;
use net_translator::component::translator::Translator;


fn main() {
    init_log();
    info!("Run module");

    let thread_pool = ThreadPool::with_name("worker".into(), 20);

    //FIXME Currently OCP is violated. The modules should be scanned based on dependencies, iterate through it and start it dynamically
    let config = net_agent::config::Config::builder().build().expect("read config error");
    Capture::new(thread_pool.clone(), config).run();

    let config = net_hub::config::Config::builder().build().expect("read config error");
    Hub::new(thread_pool.clone(), config).run();

    let config = net_translator::config::Config::builder().build().expect("read config error");
    Translator::new(thread_pool.clone(), config).run();

    let config = net_timescale::config::Config::builder().build().expect("read config error");
    Timescale::new(thread_pool.clone(), config).run();

    thread_pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}
