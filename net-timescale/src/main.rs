use log::info;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2_postgres::{PostgresConnectionManager};
use net_timescale::component::timescale::Timescale;


use native_tls::{TlsConnector, Identity};
use postgres_native_tls::MakeTlsConnector;
use std::fs;

fn main() {
    init_log();
    info!("Run module");
    let pem = fs::read("src/.ssl/client.crt").unwrap();
    let key = fs::read("src/.ssl/client.key").unwrap();
    let client = Identity::from_pkcs8(&pem, &key).unwrap();
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .identity(client)
        .build()
        .unwrap();
    let make_tls_connector = MakeTlsConnector::new(connector);
    let thread_pool = ThreadPool::with_name("worker".into(), 5);
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:PsWDgxZb@localhost".parse().unwrap(),
        make_tls_connector,
    );
    let connection_pool = r2d2::Pool::builder().max_size(10).build(manager).unwrap();
    Timescale::new(thread_pool.clone(), connection_pool).run();

    thread_pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}