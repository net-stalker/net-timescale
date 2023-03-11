use std::thread;

use shaku::{HasComponent, module};

use net_agent::module::AgentModule;
use net_core::starter::starter::Starter;

fn main() {
    let module = AgentModule::builder().build();
    let starter = module.resolve_ref();
    starter.start();
}