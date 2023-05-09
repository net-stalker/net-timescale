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

fn build_capnp() {
    let files_to_compile = std::fs::read_dir(".capnp/").unwrap();

    for file_to_compile in files_to_compile {
        build_schema!(file_to_compile.as_ref().unwrap().path().as_path().to_str().unwrap());
    }
}

fn main() {
    build_capnp();
}