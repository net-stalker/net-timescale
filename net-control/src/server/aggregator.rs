

pub struct Aggregator {
    clients: std::collections::HashMap<russh::ChannelId, String>
}

#[derive(PartialEq)]
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

    pub (super) fn pull_new_client(&mut self, channel: russh::ChannelId) -> Option<Result<bool, ()>> {
        self.clients.insert(channel, String::new());
        Some(Ok(true))
    }

    // When catching (receiving) a new symbol from a client return Aggregator::Full
    // Return Full::Ended if the command from client is ended or Full::NotEnded if it is not
    pub (super) fn pull_symbol_for(&mut self, channel: russh::ChannelId, data: &[u8]) -> Option<Result<Full, ()>> {
        let client_buffer = self.clients.get_mut(&channel).unwrap();
        let data_cooked = std::str::from_utf8(data).unwrap();
        client_buffer.push_str(data_cooked);

        if client_buffer.ends_with("\r") {
            Some(Ok(Full::Ended))
        } else {
            Some(Ok(Full::NotEnded))
        }
    }

    pub (super) fn pull_reset_for(&mut self, channel: russh::ChannelId) -> Option<Result<bool, ()>> {
        let client_buffer = self.clients.get_mut(&channel).unwrap();
        client_buffer.clear();
        Some(Ok(true))
    }

    pub (super) fn push_buffer_for(&self, channel: russh::ChannelId) -> Option<Result<String, ()>> {
        let client_buffer = self.clients.get(&channel).unwrap();
        Some(Ok(client_buffer.clone()))
    }
}

impl Default for Aggregator {
    fn default() -> Self {
        Self::new()
    }
}
