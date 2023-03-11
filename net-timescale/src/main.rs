use shaku::HasComponent;

use net_timescale::module::{Timescale, TimescaleModule};

fn main() {
    let module = TimescaleModule::builder().build();
    let starter = module.resolve_ref();
    starter.start().join().unwrap();
}