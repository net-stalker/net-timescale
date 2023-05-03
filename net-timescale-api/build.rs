fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix(".capnp")
        .file(".capnp/network_packet.capnp")
        .default_parent_module(vec!["capnp".into(), "network_packet".into()])
        .run()
        .expect("Error while compiling schema");


    ::capnpc::CompilerCommand::new()
        .src_prefix(".capnp")
        .file(".capnp/envelope.capnp")
        .default_parent_module(vec!["capnp".into(), "envelope".into()])
        .run()
        .expect("Error while compiling schema");
}