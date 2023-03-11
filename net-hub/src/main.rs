use shaku::HasComponent;

use net_hub::module::HubModule;

fn main() {
    let module = HubModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}
