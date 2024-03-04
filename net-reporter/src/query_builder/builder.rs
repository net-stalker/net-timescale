use sqlx::{postgres::{PgArguments, PgRow}, query::QueryAs, Encode, FromRow, Postgres};

pub struct QueryBuilder<'a, Response>
    where Response: for <'r> FromRow<'r, PgRow>
{
    pub query: QueryAs<'a, Postgres, Response, PgArguments>,
}

impl<'a, Response> QueryBuilder<'a, Response>
    where Response: for <'r> FromRow<'r, PgRow>
{
    pub fn new(initial_query: &'a str) -> Self {
        let query = sqlx::query_as(initial_query); 
        QueryBuilder {
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
}