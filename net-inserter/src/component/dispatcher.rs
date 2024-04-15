use std::{collections::HashMap, sync::Arc};

use crate::core::insert_handler::{InsertHandler, InsertHandlerCtor};

#[derive(Default, Debug)]
pub struct Dispatcher {
    insert_handlers_ctors: HashMap<&'static str, Arc<dyn InsertHandlerCtor>>,
}

impl Dispatcher {
    pub fn add_insertable(mut self, insertable_type: &'static str, inserter_ctor: Arc<dyn InsertHandlerCtor>) -> Self {
        let res = self.insert_handlers_ctors.insert(insertable_type, inserter_ctor);
        match res {
            Some(_) => log::debug!("Warning! You have overwriten {insertable_type} type inserter"),
            None => log::debug!("You have added {insertable_type} type inserter")
        }
        self
    }

    pub fn get_insert_handler(&self, insertable_type: &str) -> Option<Arc<dyn InsertHandler>> {
        let ctor = self.insert_handlers_ctors.get(insertable_type);
        if ctor.is_none() {
            return None;
        }
        let ctor = ctor.unwrap();
        Some(ctor.call())
    }
}