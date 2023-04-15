// TODO: use `PostgresParams` in PostgresQuery instead of using `postgres::types::ToSql + Sync`
pub trait PostgresParams: postgres::types::ToSql + Sync {}
pub trait PostgresQuery<'a> { 
    fn get_query_params(&self) -> (&'a str, &[&'a(dyn postgres::types::ToSql + Sync)]);
}