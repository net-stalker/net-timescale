use super::query_result::QueryResult;

pub trait Request{
    fn execute(&self, data: &[u8]) -> Result<QueryResult, &'static str>;
}
