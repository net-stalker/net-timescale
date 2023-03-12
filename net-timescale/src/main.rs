use shaku::HasComponent;

use net_timescale::module::{Timescale, NetTimescaleModule};

fn main() {
    let module = NetTimescaleModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}