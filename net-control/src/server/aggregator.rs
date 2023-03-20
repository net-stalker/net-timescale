pub struct Aggregator {
    //clients: std::collections::HashMap<, &'static str>
}

impl Aggregator {
    pub (super) fn new() -> Self {
        Aggregator {}
    }   

    pub (super) fn add_new_client(& self) -> Option<Result<(), ()>> {
        None
    }
}

impl Default for Aggregator {
    fn default() -> Self {
        Self::new()
    }
}
