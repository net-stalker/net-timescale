use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use postgres::NoTls;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Executor{
    pub connection_pool: Arc<Mutex<Pool<PostgresConnectionManager<NoTls>>>>
}

impl Executor{
    pub fn new(connection_pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Executor { connection_pool: Arc::new(Mutex::new(connection_pool)) }
    }
    pub fn execute_query<Q, R>(&self, query: Q) -> Result<R, postgres::Error>
    where
        Q: FnOnce(PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<R, postgres::Error>
    {
        let con = self.connection_pool.lock()
                .unwrap()
                .get()
                .unwrap();
        // TODO: consider using https://crates.io/crates/futures to improve perfomance
        query(con)
    }
}