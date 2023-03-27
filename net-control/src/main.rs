use net_control::server::cli_server;
use net_control::server::handlers::default_server_handler::DefaultServerHandler;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    cli_server::CLIServer::<DefaultServerHandler::default()>::builder()
        .build()
        .start_server();
}