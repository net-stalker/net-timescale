use log::info;
use threadpool::ThreadPool;
use net_inserter::component::inserter::Inserter;
use net_inserter::config::Config;

#[async_std::main]
async fn main() {
    init_log();
    info!("Run module");

    let config = Config::builder().build().expect("read config error");
    let thread_pool = ThreadPool::with_name("worker".into(), 5);

    Inserter::new(thread_pool.clone(), config).await.run().await;

    thread_pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}