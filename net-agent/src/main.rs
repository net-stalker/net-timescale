use log::{info};
use shaku::HasComponent;

use net_agent::module::NetAgentModule;

fn main() {
    env_logger::init();
    info!("Run service");

    let module = NetAgentModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}
