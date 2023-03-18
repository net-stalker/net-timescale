use net_control::server::cli_server;


//TODO: move host and port to the configuration
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    cli_server::CLIServer::new().start_server("0.0.0.0", "2222");
}