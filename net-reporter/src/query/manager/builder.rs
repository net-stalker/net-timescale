use std::collections::HashMap;

use crate::query::requester::RequestHandler;

use super::query_manager::QueryManager;

#[derive(Default)]
pub struct QueryManagerBuilder {
    request_handlers: HashMap<&'static str, Box<dyn RequestHandler>>
}

impl QueryManagerBuilder {
    pub fn add_request_handler(
        mut self,
        requester: Box<dyn RequestHandler>
    ) -> Self
    {
        //TODO: Create Error handling here
        let _ = self.request_handlers.insert(requester.get_requesting_type(), requester);
        self
    }

    pub fn build(self) -> QueryManager {
        QueryManager::new(
            self.request_handlers
        )
    }
}