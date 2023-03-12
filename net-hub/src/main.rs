use shaku::HasComponent;

use net_hub::module::NetHubModule;

fn main() {
    let module = NetHubModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}
