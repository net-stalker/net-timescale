use std::ops::Deref;

#[derive(PartialEq, Debug, Clone)]
pub (super) enum Ended {
    Ended,
    NotEnded
}

pub trait AddClient<C> {
    fn add_client (&mut self, client: C) -> Result<(), &str>;
}

pub trait ReadBufferForClient<C, S> {
    fn read(&mut self, client: C, buf: &[u8]) -> Result<(), &str>;
    fn read_with_status(&mut self, client: C, buf: &[u8]) -> Result<S, &str>;
}

pub trait IdentifyStatus<C, S> {
    fn identify_status(&self, client: C) -> Result<S, &str>;
}

//TODO: Add a way to return current (whole) buffer
//TODO: Get rid of &str Errs (Create own Error type)
pub struct Aggregator {
    clients: std::collections::HashMap<u64, Vec<u8>>
}

impl Aggregator {
    pub (super) fn new() -> Self {
        Aggregator {
            clients: std::collections::HashMap::new()
        }
    }

    pub (super) fn data(&self, client: u64) -> Result<&[u8], ()> {

        if !self.clients.contains_key(client) {
            Err(format!("ERROR::CLI::AGGEEGATOR::Client [{}]::This client currently not exist. Can not get its data.", client))
        }

        let client_buffer = self.clients.get(&client).unwrap();
        Ok(client_buffer.deref())
    }
}

impl AddClient<u64> for Aggregator {
    fn add_client (&mut self, client: C) -> Result<(), &str> {

        if self.clients.contains_key(client) {
            Err(format!("ERROR::CLI::AGGEEGATOR::Client [{}]::This client has already been connected. Can not add it to the aggregator.", client))
        }

        self.clients.insert(client, Vec::new());
        Ok(())
    }
}

impl IdentifyStatus<u64, Ended> for Aggregator {
    fn identify_status(&self, client: u64) -> Result<Ended, &str> {

        if !self.clients.contains_key(client) {
            Err(format!("ERROR::CLI::AGGEEGATOR::Client [{}]::This client currently not exist. Can not identify its status.", client))
        }

        let client_buffer = self.clients.get(&client).unwrap();
        if client_buffer.as_slice().ends_with("\r".as_bytes()) {
            Ok(Ended::Ended)
        } else {
            Ok(Ended::NotEnded)
        }
    }
}

impl ReadBufferForClient<u64, Ended> for Aggregator {
    fn read(&mut self, client: u64, buf: &[u8]) -> Result<(), &str> {

        if !self.clients.contains_key(client) {
            Err(format!("ERROR::CLI::AGGEEGATOR::Client [{}]::This client currently not exist. Can not read data for it.", client))
        }

        let client_buffer = self.clients.get_mut(&client).unwrap();
        client_buffer.append(&mut buf.to_vec());
        Ok(())
    }

    fn read_with_status(&mut self, client: u64, buf: &[u8]) -> Result<Ended, &str> {

        if !self.clients.contains_key(client) {
            Err(format!("ERROR::CLI::AGGEEGATOR::Client [{}]::This client currently not exist. Can not read data for it.", client))
        }

        let client_buffer = self.clients.get_mut(&client).unwrap();
        client_buffer.append(&mut buf.to_vec());
        
        self.identify_status(client)
    }
}

impl Default for Aggregator {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use crate::server::aggregator::{IdentifyStatus, Ended};
    use super::{Aggregator, AddClient, ReadBufferForClient};

    #[test]
    fn expect_create_empty_aggregator() {
        let aggregator = Aggregator::new();
        assert_eq!(aggregator.clients.capacity(), 0);
    }

    #[test]
    fn expect_correctly_add_new_clients() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        aggregator.add_client(client_hash);

        assert_eq!(aggregator.clients.len(), 1);
        assert_eq!(aggregator.clients.get(&client_hash).unwrap().len(), 0);


        let client_hash: u64 = 1u64;
        aggregator.add_client(client_hash);
        assert_eq!(aggregator.clients.len(), 2);
        assert_eq!(aggregator.clients.get(&client_hash).unwrap().len(), 0);
    }

    #[test]
    fn expect_correctly_read_data() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        aggregator.add_client(client_hash);
        aggregator.read(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes());

        assert_eq!(aggregator.clients.len(), 1);
        assert_ne!(aggregator.clients.get(&client_hash).unwrap().len(), 0);
    }

    #[test]
    fn expect_correctly_identify_status() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        aggregator.add_client(client_hash);
        aggregator.read(client_hash, format!("Hello from the user: {}", client_hash).as_bytes());

        assert_eq!(aggregator.identify_status(client_hash).unwrap(), Ended::NotEnded);


        let client_hash: u64 = 1u64;
        aggregator.add_client(client_hash);
        aggregator.read(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes());

        assert_eq!(aggregator.identify_status(client_hash).unwrap(), Ended::Ended);
    }

    #[test]
    fn expect_correctly_identify_status_while_reading_data() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        aggregator.add_client(client_hash);
        let status = aggregator.read_with_status(client_hash, format!("Hello from the user: {}", client_hash).as_bytes()).unwrap();

        assert_eq!(status, Ended::NotEnded);


        let client_hash: u64 = 1u64;
        aggregator.add_client(client_hash);
        let status = aggregator.read_with_status(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes()).unwrap();

        assert_eq!(status, Ended::Ended);
    }
}