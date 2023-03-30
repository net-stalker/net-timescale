use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use postgres::NoTls;
use threadpool::ThreadPool;
use std::{sync::{Arc, Mutex}, future::Future};

#[derive(Clone)]
pub struct Executor{
    // store connection pool
    pub connection_pool: Arc<Mutex<Pool<PostgresConnectionManager<NoTls>>>>
}

impl Executor{
    pub fn new(connection_pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Executor { connection_pool: Arc::new(Mutex::new(connection_pool)) }
    }
    // TODO: test the method
    pub async fn execute<Q, R>(&self, query: Q) -> Result<R, postgres::Error>
    where
        Q: FnOnce(PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<R, postgres::Error>
    {
        let con = self.connection_pool.lock()
                .unwrap()
                .get()
                .unwrap();
        query(con)
    }
}