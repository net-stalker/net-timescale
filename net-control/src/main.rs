use net_control::server::cli_server;
use net_control::server::handlers::default_server_handler::DefaultServerHandler;
use net_control::server::handlers::legasy_server_handler::ServerHandler;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

//TODO: get rid of a strange syntax
    cli_server::CLIServer::<DefaultServerHandler>::builder()
        .build()
        .start_server();
}