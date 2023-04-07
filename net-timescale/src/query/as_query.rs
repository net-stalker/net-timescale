use super::query_result;
pub trait AsQuery {
    fn execute(&self, data: &[u8]) -> Result<query_result::QueryResult, &'static str>;
}
