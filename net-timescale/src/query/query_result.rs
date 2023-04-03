use std::sync::Arc;
use postgres::Row;

pub trait ResultComponent { }
pub struct ReturnedRows {
    pub rows: Vec<Row> 
}
pub struct UpdatedRows {
    pub rows: u64
}
pub struct Error {
    pub error: postgres::Error
}
impl ResultComponent for ReturnedRows {}
impl ResultComponent for UpdatedRows {}
impl ResultComponent for Error {}

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
#[derive(Default)]
pub struct QueryResultBuilder {
    res: Option<Arc<dyn ResultComponent>>
}

impl QueryResultBuilder{
    pub fn new() -> QueryResultBuilder{
        QueryResultBuilder { res: None }
    }
    pub fn with_returned_rows(mut self, rows: ReturnedRows) -> Self {
        match self.res  {
            Some(_) => {
                log::error!("The result has alredy been set")
            },
            None => {
                self.res = Some(Arc::new(rows))
            }
        }
        self
    }
    pub fn with_updated_rows(mut self, rows: UpdatedRows) -> Self {
        match self.res  {
            Some(_) => {
                log::error!("The result has alredy been set")
            },
            None => {
                self.res = Some(Arc::new(rows))
            }
        }
        self
    }
    pub fn with_error(mut self, error: Error) -> Self {
        match self.res  {
            Some(_) => {
                log::error!("The result has alredy been set")
            },
            None => {
                self.res = Some(Arc::new(error))
            }
        }
        self
    }
    pub fn build(self) -> QueryResult {
        if let Some(result) = self.res {
            return QueryResult { result };
        }
        QueryResult { result: Arc::new(UpdatedRows { rows: 0 }) }
    }
}