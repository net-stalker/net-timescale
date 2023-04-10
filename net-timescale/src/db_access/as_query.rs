use super::query_result;
// TODO: this file has to be deleted further
pub trait AsQuery {
    fn execute(&self, data: &[u8]) -> Result<query_result::QueryResult, &'static str>;
}
