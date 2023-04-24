fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix(".capnp")
        .file(".capnp/data_to_send.capnp")
        .run()
        .expect("Error while compiling schema");
}