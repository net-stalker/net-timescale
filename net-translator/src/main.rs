use log::info;
use net_translator::module::NetTranslatorModule;
use shaku::HasComponent;

fn main() {
    env_logger::init();
    info!("Run service");

    let module = NetTranslatorModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}
