pub struct Aggregator {
    clients: std::collections::HashMap<russh::ChannelId, &'static str>
}

impl Aggregator {
    pub (super) fn new() -> Self {
        Aggregator {
            clients: std::collections::HashMap::new()
        }
    }   

    pub (super) fn add_new_client(&mut self, channel: russh::ChannelId) -> Option<Result<bool, ()>> {
        self.clients.insert(channel, "");
        Some(Ok(true))
    }
}

impl Default for Aggregator {
    fn default() -> Self {
        Self::new()
    }
}
