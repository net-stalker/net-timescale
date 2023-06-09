use net_control::server::cli_server;
use net_control::server::handlers::default_server_handler::DefaultServerHandler;
use net_control::server::handlers::legasy_server_handler::LegasyServerHandler;

fn main() {
    init_log();

//TODO: get rid of a strange syntax
    let server = cli_server::CLIServer::builder(LegasyServerHandler::default())
        .build();

    server.start_server();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}