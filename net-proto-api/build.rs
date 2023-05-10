fn main() {
    let files_to_compile = std::fs::read_dir(".capnp/").unwrap();

    for file_to_compile in files_to_compile { 
            let file_name = file_to_compile.as_ref().unwrap().path().as_path().to_str().unwrap().to_string();

    ::capnpc::CompilerCommand::new()
            .src_prefix(".capnp")
            .file(format!("{}", file_name))
            .default_parent_module(vec!["capnp".into(), file_name])
            .run()
            .expect("Error while compiling schema");
    }
}