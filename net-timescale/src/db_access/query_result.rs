use std::sync::Arc;

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
// result has to be `Option` enum because otherwise there is no way
// to construct a default Arc pointer with `dyn ResultComponent`
// let exceite method from AsQuery trait to return QueryREsult. But instead pf using a usuall builder
// us an assiciative trait instead
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