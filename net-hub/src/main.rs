use log::info;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_hub::component::hub::Hub;

fn main() {
    env_logger::init();
    info!("Run module");

    let pool = ThreadPool::with_name("worker".into(), 20);

    Hub::new(pool.clone()).run();

    pool.join();
}
