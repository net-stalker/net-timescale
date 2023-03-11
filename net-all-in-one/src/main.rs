use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use log::{info, trace};
use shaku::HasComponent;
use syn::{Expr, Item, ItemFn, ItemTrait, parse_file, visit};
use syn::visit::{Visit, visit_file};
use toml::Value;

use net_core::file::files::Files;

fn main() {
    let module = net_agent::module::AgentModule::builder().build();
    module.resolve_ref().start();

    let module = net_hub::module::HubModule::builder().build();
    module.resolve_ref().start();

    let module = net_timescale::module::TimescaleModule::builder().build();
    module.resolve_ref().start().join().unwrap();

    let module = net_translator::module::TranslatorModule::builder().build();
    module.resolve_ref().start().join().unwrap();
}