fn main() {
    let files_to_compile = std::fs::read_dir(".capnp/").unwrap();

    for file_to_compile in files_to_compile { 
            let file_path = file_to_compile.as_ref().unwrap().path().as_path().to_str().unwrap().to_string();

            let file_path_split = file_path.split('/');
            let file_name_with_type = file_path_split.last().unwrap();
            let file_name = file_name_with_type[0..file_name_with_type.find(".").unwrap()].to_owned();

            let file_parent_module = vec!["api".into(), file_name];

    ::capnpc::CompilerCommand::new()
            .src_prefix(".capnp")
            .file(file_path)
//TODO: Think about moving default parent module to the .capnp to prevent collisions and code reinclude
            .default_parent_module(file_parent_module)
            .run()
            .expect("Error while compiling schema");
    }    
}