use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use diesel::pg::PgConnection;
use std::sync::{Arc, Mutex};


pub struct PoolWrapper {
    connection_pool: Arc<Mutex<Pool<ConnectionManager<PgConnection>>>>
}
impl Clone for PoolWrapper {
    fn clone(&self) -> Self {
        Self { connection_pool: self.connection_pool.clone() }
    }
}

impl PoolWrapper {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        PoolWrapper { connection_pool: Arc::new(Mutex::new(connection_pool)) }
    }
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.connection_pool.lock()
        .unwrap()
        .get()
        .unwrap()
    }
}