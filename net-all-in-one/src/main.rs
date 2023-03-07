use std::fs::File;
use std::io::Read;
use log::{info, trace};

use syn::{Expr, Item, ItemFn, ItemTrait, parse_file, visit};
use syn::visit::{Visit, visit_file};
use walkdir::WalkDir;

mod traits;

fn main() {
    let source_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/traits.rs");

    // for entry in WalkDir::new(source_dir) {
    //     let entry = entry.unwrap();
    //     if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
    //         println!("{}", entry.path().display());
    //     }
    // }

    let mut file = File::open(source_dir).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let traits = count_traits(&content);
    println!("Number of traits {:?}", traits);
}

fn count_traits(content: &str) -> TraitCounter {
    let mut trait_counter = TraitCounter { count: 0 };
    let ast = parse_file(content).unwrap();

    visit_file(&mut trait_counter, &ast);

    trait_counter
}

#[derive(Debug)]
struct TraitCounter {
    count: usize,
}

impl Visit<'_> for TraitCounter {
    fn visit_item_trait(&mut self, item: &ItemTrait) {
        self.count += 1;
        println!("{}", item.ident);
        visit::visit_item_trait(self, item);
    }
}