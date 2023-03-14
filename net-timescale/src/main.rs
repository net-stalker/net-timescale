use log::info;
use threadpool::ThreadPool;
use net_core::starter::starter::NetComponent;

use net_timescale::component::timescale::Timescale;

fn main() {
    env_logger::init();
    info!("Run module");

    let pool = ThreadPool::with_name("worker".into(), 5);

    Timescale::new(pool.clone()).run();

    pool.join();
}