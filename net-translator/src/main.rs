use std::thread;

use shaku::HasComponent;

use net_translator::module::TranslatorModule;

fn main() {
    let module = TranslatorModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}