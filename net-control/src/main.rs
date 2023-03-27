use net_control::server::{cli_server, server_handler::ServerHandler};

fn main() {
    cli_server::CLIServer::<ServerHandler>::builder()
        .build()
        .start_server();
}