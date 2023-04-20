fn main() {
    ::capnpc::CompilerCommand::new()
        .file("data_to_send.capnp")
        .run()
        .expect("compiling schema");
}