use std::sync::Arc;
// TODO: this file is going to be deleted
pub trait QueryResultComponent { }
pub struct QueryResult {
    result: Arc<dyn QueryResultComponent>
}
impl QueryResult {
    pub fn get(&self) -> Arc<dyn QueryResultComponent> {
        self.result.clone()
    }
    pub fn builder() -> QueryResultBuilder {
        QueryResultBuilder::new()
    }
}
pub struct QueryResultBuilder {
    result: Option<Result<Arc<dyn QueryResultComponent>, &'static str>>
}
impl QueryResultBuilder{
    pub fn new() -> QueryResultBuilder{
        QueryResultBuilder { result: None }
    }
    pub fn with_result(mut self, res: Arc<dyn QueryResultComponent>) -> Self {
        self.result = Some(Ok(res));
        self
    }
    pub fn with_error(mut self, error: &'static str) -> Self {
        self.result = Some(Err(error));
        self
    }
    pub fn build(self) -> Result<QueryResult, &'static str>  {
        match self.result {
            Some(res) => {
                match res {
                    Ok(result) => {
                        Ok(QueryResult { result })
                    },
                    Err(error) => {
                        Err(error)
                    }
                }
            },
            None => Err("No result has been set up")
        }
    }
}