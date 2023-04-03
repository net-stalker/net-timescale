use super::query_result::QueryResult;

pub trait Request{
    fn execute(&self, data: Vec<u8>) -> QueryResult;
}
