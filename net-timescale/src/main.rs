use log::info;
use postgres::NoTls;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2_postgres::{PostgresConnectionManager};
use net_timescale::component::timescale::Timescale;

fn main() {
    env_logger::init();
    info!("Run module");

    let thread_pool = ThreadPool::with_name("worker".into(), 5);
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:PsWDgxZb@localhost".parse().unwrap(),
        NoTls
    );
    let connection_pool = r2d2::Pool::builder().max_size(10).build(manager).unwrap();
    Timescale::new(thread_pool.clone(), connection_pool).run();

    thread_pool.join();
}