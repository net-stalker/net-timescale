use log::{info};
use shaku::HasComponent;

use net_agent::module::AgentModule;

fn main() {
    env_logger::init();
    info!("Run service");

    let module = AgentModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}
