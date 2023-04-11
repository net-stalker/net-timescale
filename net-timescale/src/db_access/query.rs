// TODO: use `PostgresParams` in PostgresQuery instead of using `postgres::types::ToSql + Sync`
pub trait PostgresParams: postgres::types::ToSql + Sync {}
pub trait PostgresQuery { 
    fn get_query(&self) -> (String, Vec<&(dyn postgres::types::ToSql + Sync)>);
}