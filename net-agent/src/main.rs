use log::info;
use threadpool::ThreadPool;

use net_agent::component::capture::Capture;
use net_core::layer::NetComponent;

fn main() {
    env_logger::init();
    info!("Run module");

    let pool = ThreadPool::with_name("worker".into(), 2);

    Capture::new(pool.clone()).run();

    pool.join();
}
