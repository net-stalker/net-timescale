use net_control::server::cli_server;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    cli_server::CLIServer::default().start_server();
}