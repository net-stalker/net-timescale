use log::info;
use shaku::HasComponent;

fn main() {
    env_logger::init();
    info!("Run service");

    let module = net_hub::module::HubModule::builder().build();
    module.resolve_ref().start();

    let module = net_agent::module::AgentModule::builder().build();
    module.resolve_ref().start();

    let module = net_timescale::module::TimescaleModule::builder().build();
    module.resolve_ref().start();

    let module = net_translator::module::TranslatorModule::builder().build();
    module.resolve_ref().start().join().unwrap();
}
