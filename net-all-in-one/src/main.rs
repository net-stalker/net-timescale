use log::info;
use shaku::HasComponent;

fn main() {
    env_logger::init();
    info!("Run service");

    //FIME Currently OCP is violated. The modules should be scanned based on dependencies, iterate through it and start it dynamically
    let module = net_hub::module::NetHubModule::builder().build();
    module.resolve_ref().start();

    let module = net_agent::module::NetAgentModule::builder().build();
    module.resolve_ref().start();

    let module = net_timescale::module::NetTimescaleModule::builder().build();
    module.resolve_ref().start();

    let module = net_translator::module::NetTranslatorModule::builder().build();
    module.resolve_ref().start().join().unwrap();
}
