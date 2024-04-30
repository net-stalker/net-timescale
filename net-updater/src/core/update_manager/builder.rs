use std::collections::HashMap;

use super::manager::UpdateManager;
use super::update_handler::UpdateHandler;

#[derive(Default)]
pub struct UpdateManagerBuilder {
    update_handlers: HashMap<&'static str, Box<dyn UpdateHandler>>
}

impl UpdateManagerBuilder {
    pub fn add_request_handler(
        mut self,
        updater: Box<dyn UpdateHandler>
    ) -> Self
    {
        //TODO: Create Error handling here
        let _ = self.update_handlers.insert(updater.get_updating_request_type(), updater);
        self
    }

    pub fn build(self) -> UpdateManager {
        UpdateManager::new(
            self.update_handlers
        )
    }
}