use std::sync::Arc;

use sqlx::{postgres::{PgArguments, PgRow}, query::QueryAs, Encode, FromRow, Pool, Postgres};

pub struct SqlxQueryBuilderWrapper<'a, Response>
    where Response: for <'r> FromRow<'r, PgRow> + Send + Unpin
{
    pub query: QueryAs<'a, Postgres, Response, PgArguments>,
}

impl<'a, Response> SqlxQueryBuilderWrapper<'a, Response>
    where Response: for <'r> FromRow<'r, PgRow> + Send + Unpin
{
    pub fn new(initial_query: &'a str) -> Self {
        let query = sqlx::query_as(initial_query); 
        SqlxQueryBuilderWrapper {
            query,
        }
    }

    pub fn add_param<Param: for<'b> Encode<'b, Postgres> + sqlx::Type<Postgres>>(mut self, value: &'a Param) -> Self
    where
        &'a Param: Send,
    {
        self.query = self.query.bind(value);
        self
    }

    pub fn add_option_param<Param: for<'b> Encode<'b, Postgres> + sqlx::Type<Postgres>>(
        mut self,
        value: Option<&'a Param>,
    ) -> Self 
    where 
        &'a Param: Send,
    {
        if let Some(value) = value {
            self.query = self.query.bind(value);
        }
        self
    }
    pub async fn execute_query(
        self,
        connection_pool: Arc<Pool<Postgres>>,
     ) -> Result<Vec<Response>, sqlx::Error> {
        self.query.fetch_all(connection_pool.as_ref()).await
    }
}