use log::{info, warn};
use threadpool::ThreadPool;

use net_core::layer::NetComponent;
use net_hub::component::hub::Hub;
use net_hub::config::Config;

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

    let pool = ThreadPool::with_name("worker".into(), 20);

    Hub::new(pool.clone()).run();

    pool.join();
}
