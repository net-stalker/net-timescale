use log::info;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_translator::component::translator::Translator;

pub mod data_to_send_capnp {
    include!(concat!(env!("OUT_DIR"), "/data_to_send_capnp.rs"));
}

fn main() {
    env_logger::init();
    info!("Run module");

    let pool = ThreadPool::with_name("worker".into(), 5);

    Translator::new(pool.clone()).run();

    pool.join();
}
