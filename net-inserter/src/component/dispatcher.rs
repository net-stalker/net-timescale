use std::collections::HashMap;

use crate::core::insert_handler::InsertHandler;

#[derive(Debug)]
pub struct Dispatcher {
    insert_handlers_ctors: HashMap<&'static str, Box<dyn InsertHandler>>,
}

impl Dispatcher {
    fn new(insert_handlers_ctors: HashMap<&'static str, Box<dyn InsertHandler>>) -> Self {
        Self { insert_handlers_ctors }
    }

    pub fn builder() -> DispatcherBuilder {
        DispatcherBuilder::default()
    }

    pub fn get_insert_handler(&self, insertable_type: &str) -> Option<&dyn InsertHandler> {
        match self.insert_handlers_ctors.get(insertable_type) {
            Some(boxed) => Some(boxed.as_ref()),
            None => None,
        }
    }
}

#[derive(Default, Debug)]
pub struct DispatcherBuilder {
    insert_handlers_ctors: HashMap<&'static str, Box<dyn InsertHandler>>,
}

impl DispatcherBuilder {
    pub fn add_insert_handler(mut self, insert_handler: Box<dyn InsertHandler>) -> Self {
        let insertable_type = insert_handler.get_insertable_data_type();
        let res = self.insert_handlers_ctors.insert(insertable_type, insert_handler);
        match res {
            Some(_) => log::debug!("Warning! You have overwriten {insertable_type} type insert handler"),
            None => log::debug!("You have added {insertable_type} type insert handler")
        }
        self
    }

    pub fn build(self) -> Dispatcher {
        Dispatcher::new(self.insert_handlers_ctors)
    }
}
