use log::info;
use postgres::NoTls;
use std::io;
use std::io::prelude::*;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2_postgres::{PostgresConnectionManager};
use net_timescale::component::timescale::Timescale;

use postgres::config::Config;

use native_tls::{Certificate, TlsConnector, Identity};
use postgres_native_tls::MakeTlsConnector;
use std::fs::{self, File};

fn main() {
    env_logger::init();
    info!("Run module");
    let pem = fs::read("src/.ssl/client.crt").unwrap();
    let key = fs::read("src/.ssl/client.key").unwrap();

    let root = fs::read("timescaledb/certs/root.crt").unwrap();
    let cert = Certificate::from_pem(root.as_slice()).unwrap();
    let client = Identity::from_pkcs8(&pem, &key).unwrap();
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .identity(client)
        .build()
        .unwrap();
    let make_tls_connector = MakeTlsConnector::new(connector); 
    let thread_pool = ThreadPool::with_name("worker".into(), 5);
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:PsWDgxZb@localhost".parse().unwrap(),
        make_tls_connector
    );
    let connection_pool = r2d2::Pool::builder().max_size(10).build(manager).unwrap();
    Timescale::new(thread_pool.clone(), connection_pool).run();

    thread_pool.join();
}