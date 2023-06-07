use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use net_timescale::component::timescale::Timescale;
use native_tls::{TlsConnector, Identity};
use postgres_native_tls::MakeTlsConnector;

use r2d2_postgres::PostgresConnectionManager;
use postgres::{
    NoTls,
    Socket,
    tls::{
        MakeTlsConnect,
        TlsConnect
    }
};
enum Pool {
    NoTlsPool(r2d2::Pool<PostgresConnectionManager<NoTls>>),
    TlsPool(r2d2::Pool<PostgresConnectionManager<MakeTlsConnector>>)
}

fn configure_tls(client_key: Vec<u8>, client_crt: Vec<u8>, accept_invalid_certs: bool, connection_string: String)
    -> Pool {
    if client_crt.is_empty() && client_key.is_empty() {
        let connection_manager = PostgresConnectionManager::new(
            connection_string.parse().unwrap(),
            NoTls
        );
        return Pool::NoTlsPool(r2d2::Pool::builder().max_size(10).build(connection_manager).unwrap());
    }
    let client = Identity::from_pkcs8(&client_crt, &client_key).unwrap();
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(accept_invalid_certs)
        .identity(client)
        .build()
        .unwrap();
    let tls_connector = MakeTlsConnector::new(connector);
    let connection_manager = PostgresConnectionManager::new(
        connection_string.parse().unwrap(),
        tls_connector
    );
    Pool::TlsPool(r2d2::Pool::builder().max_size(10).build(connection_manager).unwrap())
}


fn main() {
    env_logger::init();
    log::info!("Run module");
    // TODO: add configuration
    let pem = std::fs::read("src/.ssl/client.crt").unwrap();
    let key = std::fs::read("src/.ssl/client.key").unwrap();
    let connection_string = "postgres://postgres:PsWDgxZb@localhost".to_string();
    let accept_invalid_certs = true;
    let thread_pool = ThreadPool::with_name("worker".into(), 5);
    let connection_pool = configure_tls(
        key,
        pem,
        accept_invalid_certs,
        connection_string
    );
    match connection_pool {
        Pool::NoTlsPool(pool) => {
            Timescale::new(thread_pool.clone(), pool).run();
        },
        Pool::TlsPool(pool) => {
            Timescale::new(thread_pool.clone(), pool).run();
        }
    }

    thread_pool.join();
}