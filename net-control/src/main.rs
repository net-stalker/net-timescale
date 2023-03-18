use net_control::server::cli_server;

fn main() {
    cli_server::CLIServer::default().start_server();
}