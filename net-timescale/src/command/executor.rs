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
    // basically execute_query has to receive trait. It has the next structure
    //
    // "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
    // &[&frame_time, &src_addr, &dst_addr, &json_value]
    // Such queries has to be tested
    pub fn execute_query<Q, R>(&self, query: Q) -> Result<R, postgres::Error>
    where
        Q: FnOnce(PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<R, postgres::Error>
    {
        let con = self.connection_pool.lock()
                .unwrap()
                .get()
                .unwrap();
        // TODO: consider using https://crates.io/crates/futures to improve perfomance
        // it has to be executed like - con.execute(recived structure)
        query(con)
    }
}