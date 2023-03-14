use log::info;
use threadpool::ThreadPool;

use net_agent::component::capture::Capture;
use net_core::layer::NetComponent;
use net_hub::component::hub::Hub;
use net_timescale::component::timescale::Timescale;
use net_translator::component::translator::Translator;

fn main() {
    env_logger::init();
    info!("Run module");

    let pool = ThreadPool::with_name("worker".into(), 20 );

    //FIXME Currently OCP is violated. The modules should be scanned based on dependencies, iterate through it and start it dynamically
    Capture::new(pool.clone()).run();
    Hub::new(pool.clone()).run();
    Translator::new(pool.clone()).run();
    Timescale::new(pool.clone()).run();

    pool.join();
}
