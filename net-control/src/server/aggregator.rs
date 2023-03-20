pub struct Aggregator {
    clients: std::collections::HashMap<russh::ChannelId, String>
}

pub (super) enum Full {
    Ended,
    NotEnded
}

impl Aggregator {
    pub (super) fn new() -> Self {
        Aggregator {
            clients: std::collections::HashMap::new()
        }
    }   

    pub (super) fn add_new_client(&mut self, channel: russh::ChannelId) -> Option<Result<bool, ()>> {
        self.clients.insert(channel, String::new());
        Some(Ok(true))
    }

    // When catching (receiving) a new symbol from a client return Aggregator::Full
    // Return Full::Ended if the command from client is ended or Full::NotEnded if it is not
    pub (super) fn catch_symbol_for(&mut self, channel: russh::ChannelId, data: &[u8]) -> Option<Result<Full, ()>> {
        let client_buffer = self.clients.get_mut(&channel).unwrap();
        let data_cooked = std::str::from_utf8(data).unwrap();
        client_buffer.push_str(data_cooked);

        if client_buffer.ends_with("\r") {
            Some(Ok(Full::Ended))
        } else {
            Some(Ok(Full::NotEnded))
        }
    }
}

impl Default for Aggregator {
    fn default() -> Self {
        Self::new()
    }
}
