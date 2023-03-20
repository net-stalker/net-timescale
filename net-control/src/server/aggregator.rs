pub struct Aggregator {
    //clients: std::collections::HashMap<, &'static str>
}

impl Aggregator {
    pub (super) fn new() -> Self {
        Aggregator {}
    }   
}

impl Default for Aggregator {
    fn default() -> Self {
        Self::new()
    }
}
