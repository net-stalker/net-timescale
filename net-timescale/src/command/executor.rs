use r2d2::{Pool, PooledConnection, ManageConnection};
use std::sync::{Arc, Mutex};

use crate::db_access::query;

pub struct Executor<M>
where M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    pub connection_pool: Arc<Mutex<Pool<M>>>
}
impl<M> Clone for Executor<M>
where M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    fn clone(&self) -> Self {
        Self { connection_pool: self.connection_pool.clone() }
    }
}

impl<M> Executor<M>
where M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    pub fn new(connection_pool: Pool<M>) -> Self {
        Executor { connection_pool: Arc::new(Mutex::new(connection_pool)) } //, phantom: &PhantomData }
    }
    fn get_connection(&self) -> PooledConnection<M> {
        self.connection_pool.lock()
        .unwrap()
        .get()
        .unwrap()
    }
    pub fn execute<'a, Q>(&self, query: &'a Q) -> Result<u64, postgres::Error>
    where
        Q: query::PostgresQuery<'a>
    {
        let (query_string, params) = query.get_query_params();
        self.get_connection().execute(query_string, params)
    }
    pub fn query<'a, Q>(&self, query: &'a Q) -> Result<Vec<postgres::Row>, postgres::Error>
    where
        Q: query::PostgresQuery<'a>
    {
        let (query_string, params) = query.get_query_params();
        self.get_connection().query(query_string, params)
    }
}