use net_control::server::cli_server;
use net_control::server::handlers::default_server_handler::DefaultServerHandler;

fn main() {
    cli_server::CLIServer::<DefaultServerHandler>::builder()
        .build()
        .start_server();
}