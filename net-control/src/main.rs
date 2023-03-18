use net_control::server::control_server;


//TODO: move host and port to the configuration
fn main() {
    control_server::CLIServer::new().start_server("0.0.0.0", "2222");
}