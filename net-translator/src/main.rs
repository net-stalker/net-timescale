use log::info;
use threadpool::ThreadPool;
use net_core::starter::starter::NetComponent;

use net_translator::component::translator::Translator;

fn main() {
    env_logger::init();
    info!("Run module");

    let pool = ThreadPool::with_name("worker".into(), 5);

    Translator::new(pool.clone()).run();

    pool.join();
}
