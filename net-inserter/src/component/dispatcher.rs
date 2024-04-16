use std::collections::HashMap;

use crate::core::insert_handler::InsertHandler;

#[derive(Default, Debug)]
pub struct Dispatcher {
    insert_handlers_ctors: HashMap<&'static str, Box<dyn InsertHandler>>,
}

impl Dispatcher {
    pub fn add_insert_handler(mut self, insertable_type: &'static str, inserter_ctor: Box<dyn InsertHandler>) -> Self {
        let res = self.insert_handlers_ctors.insert(insertable_type, inserter_ctor);
        match res {
            Some(_) => log::debug!("Warning! You have overwriten {insertable_type} type insert handler"),
            None => log::debug!("You have added {insertable_type} type insert handler")
        }
        self
    }

    pub fn get_insert_handler(&self, insertable_type: &str) -> Option<&dyn InsertHandler> {
        match self.insert_handlers_ctors.get(insertable_type) {
            Some(boxed) => Some(&**boxed),
            None => None,
        }
    }
}