fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix(".capnp")
        .file(".capnp/query_data.capnp")
        .default_parent_module(vec!["capnp".into(), "query_data".into()])
        .run()
        .expect("Error while compiling schema");


    ::capnpc::CompilerCommand::new()
        .src_prefix(".capnp")
        .file(".capnp/envelope.capnp")
        .default_parent_module(vec!["capnp".into(), "query_data".into()])
        .run()
        .expect("Error while compiling schema");
}