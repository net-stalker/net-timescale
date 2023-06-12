pub trait SqlQuery {
    fn get_sql_query(self) -> diesel::query_builder::SqlQuery;
}