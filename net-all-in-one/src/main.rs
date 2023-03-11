use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use log::{info, trace};
use shaku::HasComponent;
use syn::{Expr, Item, ItemFn, ItemTrait, parse_file, visit};
use syn::visit::{Visit, visit_file};
use toml::Value;

use net_core::file::files::Files;

mod traits;

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

fn find_sub_crates_path_with_net_prefix(toml_content: String) -> Vec<PathBuf> {
    let toml = toml_content.parse::<Value>().unwrap();
    toml.get("dependencies").unwrap()
        .as_table().unwrap()
        .iter()
        .filter(|&x| { x.0.contains("net-") })
        .map(|key| { key.1.get("path").unwrap() })
        .map(|path_value| { path_value.as_str().unwrap() })
        .map(|path_str| { PathBuf::from(path_str) })
        .map(|path_buf| { Path::new(env!("CARGO_MANIFEST_DIR")).join(path_buf) })
        .map(|path_buf| { path_buf.canonicalize().expect("failed to get absolute path") })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_find_sub_creates_path_with_net_prefix() {
        let toml_content = r#"
                                    [package]
                                    name = "net-all-in-one"
                                    version = "0.1.0"
                                    edition = "2021"

                                    [dependencies]
                                    net-core = { path = "../net-core" }
                                    net-agent = { path = "../net-agent" }
                                    net-hub = { path = "../net-hub" }

                                    log = "0.4"
                                    quote = "1.0.23"
                                    syn = { version = "1.0.109", features = ["parsing", "full", "visit"] }
                                    toml = "0.7.2"
                                    walkdir = "2.3.2"
        "#.to_string();

        let paths = find_sub_crates_path_with_net_prefix(toml_content);
        assert_eq!(paths, vec![PathBuf::from("/Users/dmytroscherbatuik/projects/netstalker/net-monitor/net-agent"),
                               PathBuf::from("/Users/dmytroscherbatuik/projects/netstalker/net-monitor/net-core"),
                               PathBuf::from("/Users/dmytroscherbatuik/projects/netstalker/net-monitor/net-hub"), ]);
    }

    #[test]
    fn test() {
        // let cargo_path = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        // let cargo_content = Files::read_string(cargo_path);
        let toml_content = r#"
                                    [package]
                                    name = "net-all-in-one"
                                    version = "0.1.0"
                                    edition = "2021"

                                    [dependencies]
                                    net-core = { path = "../net-core" }
                                    net-agent = { path = "../net-agent" }
                                    net-hub = { path = "../net-hub" }

                                    log = "0.4"
                                    quote = "1.0.23"
                                    syn = { version = "1.0.109", features = ["parsing", "full", "visit"] }
                                    toml = "0.7.2"
                                    walkdir = "2.3.2"
        "#.to_string();

        let dirs = find_sub_crates_path_with_net_prefix(toml_content);

        // let cargo_path = concat!(env!("CARGO_MANIFEST_DIR"), "");
        // let files = find_files(cargo_path);
        let files = Files::find_rs_files(dirs.get(0).unwrap());
        println!("Rust files: {:?}", files);

        // files.into_iter()
        //     .for_each(|path| {
        //         let content = Files::read_string(path.as_str());
        //         let traits = find_trait(&content);
        //         println!("Number of traits {:?}", traits);
        //     })
    }
}