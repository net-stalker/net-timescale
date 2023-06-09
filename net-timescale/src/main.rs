use log::info;
use std::sync::Arc;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use net_timescale::component::timescale::Timescale;
use postgres::{
    NoTls,
    Socket,
    tls::{
        MakeTlsConnect,
        TlsConnect
    }
};
use net_timescale::tls_configuration::{
    ConnectionFactory,
    Pool,
    tls_factory::TlsConnectionFactory,
    no_tls_factory::NoTlsConnectionFactory,
};


fn get_factory() -> Arc<dyn ConnectionFactory> {
    // TODO: read this info from config
    let enable_tls = true;
    let crt_path = "src/.ssl/client.crt".to_owned();
    let key_path = "src/.ssl/client.key".to_owned();
    let connection_string = "postgres://postgres:PsWDgxZb@localhost".to_string();
    let max_connection_size = 10;
    let accept_invalid_certs = true;

    match enable_tls {
        true => {
            TlsConnectionFactory::builder()
                .with_crt(crt_path)
                .with_key(key_path)
                .with_connection_string(connection_string)
                .with_max_connection_size(max_connection_size)
                .accept_invalid_certs(accept_invalid_certs)
                .build()
                .into_inner()
        }
        false => {
            NoTlsConnectionFactory::builder()
                .with_max_connection_size(max_connection_size)
                .with_connection_string(connection_string)
                .build()
                .into_inner()
        }
    }
}



fn main() {
    init_log();
    info!("Run module");
    // TODO: add configuration
    let thread_pool = ThreadPool::with_name("worker".into(), 5);
    let factory = get_factory();
    match factory.create_pool() {
        Pool::NoTlsPool(pool) => {
            Timescale::new(thread_pool.clone(), pool).run();
        },
        Pool::TlsPool(pool) => {
            Timescale::new(thread_pool.clone(), pool).run();
        }
    }
    thread_pool.join();
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}