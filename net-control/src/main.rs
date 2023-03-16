use net_control::server::control_server;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    control_server::CLIServer::new().start_server("0.0.0.0", "2222");
}