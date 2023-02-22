use net_control::server::control_server;

fn main() {
    control_server::CLIServer::new().start_server();
}