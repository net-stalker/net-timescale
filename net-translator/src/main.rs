use log::info;
use net_translator::module::TranslatorModule;
use shaku::HasComponent;

fn main() {
    env_logger::init();
    info!("Run service");

    let module = TranslatorModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}
