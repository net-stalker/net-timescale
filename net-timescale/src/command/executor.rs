use std::sync::{Arc, Mutex};
use sqlx::{
    postgres::PgPoolOptions,
    Database,
    Pool
};

pub struct PoolWrapper<DB>
where DB: Database
{
    connection_pool: Pool<DB>,
}

impl<DB> Clone for PoolWrapper<DB>
where DB: Database
{
    fn clone(&self) -> Self {
        Self {
            connection_pool: self.connection_pool.clone(),
        }
    }
}
impl<DB> PoolWrapper<DB>
where DB: Database
{
    pub fn new(connection_pool: Pool<DB>) -> Self {
        PoolWrapper { connection_pool }
    }
    pub fn into_inner(self) -> Arc<Self> { Arc::new(self) }
    pub async fn get_connection(&self) -> &Pool<DB> {
        &self.connection_pool
    }
}