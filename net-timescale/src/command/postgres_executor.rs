use r2d2::{Pool, PooledConnection, ManageConnection};
use std::{sync::{Arc, Mutex}, marker::PhantomData};
use crate::db_access::query;
use super::executor::Executor;

pub struct PostgresExecutor<'a, M>
where M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    pub connection_pool: Arc<Mutex<Pool<M>>>,
    phantom: &'a PhantomData<M>,
}
impl<'a, M> Clone for PostgresExecutor<'a, M>
where M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    fn clone(&self) -> Self {
        Self { connection_pool: self.connection_pool.clone(), phantom: self.phantom }
    }
}

impl<'a, M> PostgresExecutor<'a, M>
where M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    pub fn new(connection_pool: Pool<M>) -> Self {
        PostgresExecutor { connection_pool: Arc::new(Mutex::new(connection_pool)), phantom: &PhantomData }
    }
    fn get_connection(&self) -> PooledConnection<M> {
        self.connection_pool.lock()
        .unwrap()
        .get()
        .unwrap()
    }
}

impl<'a, M> Executor<'a> for PostgresExecutor<'a, M>
where M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    type Q = dyn for<'b> query::PostgresQuery<'b>;

    type E = postgres::Error;

    type R = Vec<postgres::Row>;

    fn execute(&self, query: &'a Self::Q) -> Result<u64, Self::E> {
        let (query_string, params) = query.get_query_params();
        self.get_connection().execute(query_string, params)
    }

    fn query(&self, query: &'a Self::Q) -> Result<Self::R, Self::E> {
        let (query_string, params) = query.get_query_params();
        self.get_connection().query(query_string, params)
    }
}