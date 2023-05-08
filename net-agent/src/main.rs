use log::{info, warn};
use threadpool::ThreadPool;

use net_agent::component::capture::Capture;
use net_agent::config::Config;
use net_core::layer::NetComponent;

fn main() {
    env_logger::init();
    info!("run module");

    match Config::builder().build() {
        Ok(config) => {
            info!("{}", config)
        }
        Err(e) => {
            warn!("{}", e)
        }
    }

    let pool = ThreadPool::with_name("worker".into(), 2);

    Capture::new(pool.clone()).run();

    pool.join();
}
