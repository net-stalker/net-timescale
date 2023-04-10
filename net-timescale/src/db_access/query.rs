// trait Query{
//     type QUERY;
//     type PARAMS;
// }
pub trait PostgresParams: postgres::types::ToSql + Sync {}
pub trait PostgresQuery<'a> { 
    fn get_query(&self) -> (&'a str, &'a[&'a(dyn postgres::types::ToSql + Sync)]);
}
// impl<'a> Query for dyn PostgresQuery<'a> {
//     type QUERY = &'a str;

//     type PARAMS = &'a[&'a(dyn PostgresParams)];
// }