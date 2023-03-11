use syn::{ItemImpl, ItemTrait, parse_file, Path, visit};
use syn::visit::{Visit, visit_file};

#[derive(Debug)]
struct TraitHunter {
    count: usize,
}

impl TraitHunter {
    fn find_trait(content: &str) -> TraitHunter {
        let mut trait_counter = TraitHunter { count: 0 };
        let ast = parse_file(content).unwrap();

        visit_file(&mut trait_counter, &ast);

        trait_counter
    }

    // fn find_trait_implementations(source: &str, trait_path: &Path) -> Vec<ItemImpl> {
    //     let syntax_tree = parse_file(source).unwrap();
    //     let mut impls = vec![];
    //
    //     for item in syntax_tree.items {
    //         if let syn::Item::Impl(imp) = item {
    //             if let Some(trait_ref) = imp.trait_.as_ref() {
    //                 if let Some(path) = trait_ref.path.get_ident() {
    //                     if path == &trait_path.get_ident().unwrap() {
    //                         impls.push(imp);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     impls
    // }
}

impl Visit<'_> for TraitHunter {
    fn visit_item_trait(&mut self, item: &ItemTrait) {
        self.count += 1;
        println!("{}", item.ident);
        visit::visit_item_trait(self, item);
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;
    use super::*;

    #[test]
    fn expected_find_trait() {
        let content = r#"
                                trait Starter {}
        "#;

        let traits = TraitHunter::find_trait(content);
        assert_eq!(traits.count, 1)
    }

    #[test]
    fn find_trair_impl() {
        let source = r#"
    struct MyStruct {}

    impl Debug for MyStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            unimplemented!()
        }
    }

    impl Debug for i32 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            unimplemented!()
        }
    }
"#;

        let trait_path = parse_quote!(std::fmt::Debug);

        let implementations = find_trait_implementations(source, &trait_path).unwrap();

        assert_eq!(implementations.len(), 2);
    }
}