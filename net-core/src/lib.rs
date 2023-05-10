pub mod capture;
pub mod transport;
pub mod file;
pub mod jsons;
pub mod layer;
pub mod serialization;

#[macro_export]
macro_rules! test_resources {
        ($fname:expr) => (
        // The environment variable CARGO_MANIFEST_DIR provide a stable base point to reference other files.
        // Here, we assume that there's a test/resources directory at the top level of the crate
        concat!(env!("CARGO_MANIFEST_DIR"), "/test/resources/", $fname)
        )
}

#[macro_export]
macro_rules! build_schema {
    ($file_name:expr) => {
        ::capnpc::CompilerCommand::new()
            .src_prefix(".capnp")
            .file(format!("{}", $file_name))
            .default_parent_module(vec!["capnp".into(), $file_name.into()])
            .run()
            .expect("Error while compiling schema");
    }
}

#[macro_export]
macro_rules! build_capnp {
    () => {
        let files_to_compile = std::fs::read_dir(".capnp/").unwrap();
    
        for file_to_compile in files_to_compile { 
                net_core::build_schema!(file_to_compile.as_ref().unwrap().path().as_path().to_str().unwrap());
        }
    }
}