use std::ops::DerefMut;
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};
use diesel::pg::PgConnection;
use std::sync::{Arc, Mutex};
use diesel::query_builder::SqlQuery;
use diesel::RunQueryDsl;


pub struct Executor {
    pub connection_pool: Arc<Mutex<Pool<ConnectionManager<PgConnection>>>>
}
impl Clone for Executor {
    fn clone(&self) -> Self {
        Self { connection_pool: self.connection_pool.clone() }
    }
}

impl Executor {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Executor { connection_pool: Arc::new(Mutex::new(connection_pool)) }
    }
    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.connection_pool.lock()
        .unwrap()
        .get()
        .unwrap()
    }
    pub fn execute(&self, query: SqlQuery) -> diesel::QueryResult<usize>
    {
        query.execute(self.get_connection().deref_mut())
    }
    // TODO: investigate more about mapping structures with the data returned from queries
    // pub fn query<'a, Q>(&self, query: &'a Q) -> Result<Vec<postgres::Row>, postgres::Error>
    // where
    //     Q: sql_query::PostgresQuery<'a>
    // {
    //     let (query_string, params) = query.get_query_params();
    //     self.get_connection().query(query_string, params)
    // }
}