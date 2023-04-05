use std::sync::Arc;

pub trait ResultComponent { }
pub struct QueryResult {
    result: Arc<dyn ResultComponent>
}
impl QueryResult {
    pub fn get(&self) -> Arc<dyn ResultComponent> {
        self.result.clone()
    }
    pub fn builder() -> QueryResultBuilder {
        QueryResultBuilder::new()
    }
}
// result has to be `Option` enum because otherwise there is no way
// to construct a default Arc pointer with `dyn ResultComponent`
pub struct QueryResultBuilder {
    result: Option<Arc<dyn ResultComponent>>
}
impl QueryResultBuilder{
    pub fn new() -> QueryResultBuilder{
        QueryResultBuilder { result: None }
    }
    pub fn with_result(mut self, res: Arc<dyn ResultComponent>) -> Self{
        self.result = Some(res);
        self
    }
    pub fn build(self) -> Result<QueryResult, &'static str>  {
        match self.result {
            Some(result) => return Ok( QueryResult { result }),
            None => return Err("No result has been set up")
        }
    }
}