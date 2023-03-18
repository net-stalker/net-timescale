use net_control::server::cli_server;


//TODO: move host and port to the configuration
fn main() {
    cli_server::CLIServer::new().start_server("0.0.0.0", "2222");
}