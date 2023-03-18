use net_control::server::cli_server;


//TODO: move host and port to the configuration
fn main() {
    cli_server::CLIServer::default().start_server();
}