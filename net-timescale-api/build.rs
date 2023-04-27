fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix(".capnp")
        .file(".capnp/data_to_send.capnp")
        .default_parent_module(vec!["capnp".into(), "data_to_send".into()])
        .run()
        .expect("Error while compiling schema");
}