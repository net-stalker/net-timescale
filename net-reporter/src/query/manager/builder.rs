use std::collections::HashMap;

use crate::query::requester::Requester;

use super::manager::QueryManager;

pub struct QueryManagerBuilder {
    requesters: HashMap<&'static str, Box<dyn Requester>>
}

impl Default for QueryManagerBuilder {
    fn default() -> Self {
        Self {
            requesters: HashMap::new()
        }
    }
}

impl QueryManagerBuilder {
    pub fn add_chart_generator(
        mut self,
        requester: Box<dyn Requester>
    ) -> Self
    {
        //TODO: Create Error handling here
        let _ = self.requesters.insert(requester.get_requesting_type(), requester);
        self
    }

    pub fn build(self) -> QueryManager {
        QueryManager::new(
            self.requesters
        )
    }
}