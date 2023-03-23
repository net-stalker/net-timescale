use net_control::server::{cli_server, server_config::ServerConfig, control_server::ControlServer, server_handler::ServerHandler};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    cli_server::CLIServer::builder()
        .with_config(ServerConfig::default())
        .with_server(ControlServer::builder().with_handler(ServerHandler::default()).build())
        .build()
        .start_server();
}