use std::ops::Deref;

use crate::core::aggregator_errors::AggregatorError;
use crate::core::aggregator_errors::AggregatorErrorKind;
use crate::core::aggregator_errors::AggregatorErrorContext;
use crate::core::aggregator_errors::AggregationContext;



pub type Result<T> = std::result::Result<T, AggregatorError>;

#[derive(PartialEq, Debug, Clone)]
pub enum Ended {
    Ended,
    NotEnded
}

pub trait AddClient<C> {
    fn add_client (&mut self, client: C) -> Result<()>;
}

pub trait ReadBufferForClient<C, S> {
    fn read(&mut self, client: C, buf: &[u8]) -> Result<()>;
    fn read_with_status(&mut self, client: C, buf: &[u8]) -> Result<S>;
}

pub trait IdentifyStatus<C, S> {
    fn identify_status(&self, client: C) -> Result<S>;
}

//TODO: Add a way to return current (whole) buffer
pub struct Aggregator {
    clients: std::collections::HashMap<u64, Vec<u8>>
}

impl Aggregator {
    pub (super) fn new() -> Self {
        Aggregator {
            clients: std::collections::HashMap::new()
        }
    }

    pub (super) fn data(&self, client: u64) -> Result<&[u8]> {

        if !self.clients.contains_key(&client) {
            return Err(AggregatorError{
                kind: AggregatorErrorKind::ClientNotExist,
                context: AggregatorErrorContext{
                    context: AggregationContext::GetBufferError,
                    user: client
                }
            })
        }

        if self.identify_status(client).unwrap() != Ended::Ended {
            return Err(AggregatorError{
                kind: AggregatorErrorKind::ClientMsgIsNotEnded,
                context: AggregatorErrorContext{
                    context: AggregationContext::GetBufferError,
                    user: client
                }
            })
        }

        let client_buffer = self.clients.get(&client).unwrap();
        Ok(client_buffer.deref())
    }

    pub (super) fn erase_data(&mut self, client: u64) -> Result<()> {

        if !self.clients.contains_key(&client) {
            return Err(AggregatorError{
                kind: AggregatorErrorKind::ClientNotExist,
                context: AggregatorErrorContext{
                    context: AggregationContext::ErasingArror,
                    user: client
                }
            })
        }

        let client_buffer = self.clients.get_mut(&client).unwrap();
        client_buffer.clear();
        Ok(())
    }
}

impl AddClient<u64> for Aggregator {
    fn add_client (&mut self, client: u64) -> Result<()> {

        if self.clients.contains_key(&client) {
            return Err(AggregatorError{
                kind: AggregatorErrorKind::ClientAlreadyExists,
                context: AggregatorErrorContext{
                    context: AggregationContext::AddingClientError,
                    user: client
                }
            })
        }

        self.clients.insert(client, Vec::new());
        Ok(())
    }
}

impl IdentifyStatus<u64, Ended> for Aggregator {
    fn identify_status(&self, client: u64) -> Result<Ended> {

        if !self.clients.contains_key(&client) {
            return Err(AggregatorError{
                kind: AggregatorErrorKind::ClientNotExist,
                context: AggregatorErrorContext{
                    context: AggregationContext::StatusIdentifyingError,
                    user: client
                }
            })
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
    fn read(&mut self, client: u64, buf: &[u8]) -> Result<()> {

        if !self.clients.contains_key(&client) {
            return Err(AggregatorError{
                kind: AggregatorErrorKind::ClientNotExist,
                context: AggregatorErrorContext{
                    context: AggregationContext::AggregationError,
                    user: client
                }
            })
        }

        let client_buffer = self.clients.get_mut(&client).unwrap();
        client_buffer.append(&mut buf.to_vec());
        Ok(())
    }

    fn read_with_status(&mut self, client: u64, buf: &[u8]) -> Result<Ended> {

        if !self.clients.contains_key(&client) {
            return Err(AggregatorError{
                kind: AggregatorErrorKind::ClientNotExist,
                context: AggregatorErrorContext{
                    context: AggregationContext::AggregationError,
                    user: client
                }
            })
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
    use crate::core::aggregator_errors::AggregatorError;
    use crate::core::aggregator_errors::AggregatorErrorKind;
    use crate::core::aggregator_errors::AggregatorErrorContext;
    use crate::core::aggregator_errors::AggregationContext;

    use crate::server::aggregator::Ended;
    
    use crate::server::aggregator::Aggregator;
    
    use crate::server::aggregator::AddClient;
    use crate::server::aggregator::IdentifyStatus;
    use crate::server::aggregator::ReadBufferForClient;

    #[test]
    fn expect_create_empty_aggregator() {
        let aggregator = Aggregator::new();
        assert_eq!(aggregator.clients.capacity(), 0);
    }

    #[test]
    fn expect_correctly_add_new_clients() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        assert!(aggregator.add_client(client_hash).is_ok());

        assert_eq!(aggregator.clients.len(), 1);
        assert_eq!(aggregator.clients.get(&client_hash).unwrap().len(), 0);


        let client_hash: u64 = 1u64;
        assert!(aggregator.add_client(client_hash).is_ok());
        assert_eq!(aggregator.clients.len(), 2);
        assert_eq!(aggregator.clients.get(&client_hash).unwrap().len(), 0);
    }

    #[test]
    fn expect_error_on_add_existing_client() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        assert!(aggregator.add_client(client_hash).is_ok());

        assert_eq!(aggregator.clients.len(), 1);
        assert_eq!(aggregator.clients.get(&client_hash).unwrap().len(), 0);

        let client_adding_result = aggregator.add_client(client_hash);
        assert!(client_adding_result.is_err());

        assert_eq!(client_adding_result.unwrap_err(), 
            AggregatorError{
                kind: AggregatorErrorKind::ClientAlreadyExists,
                context: AggregatorErrorContext{
                    context: AggregationContext::AddingClientError,
                    user: client_hash
                }
            });

        assert_eq!(aggregator.clients.len(), 1);
        assert_eq!(aggregator.clients.get(&client_hash).unwrap().len(), 0);
    }

    #[test]
    fn expect_correctly_read_data() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        assert!(aggregator.add_client(client_hash).is_ok());
        assert!(aggregator.read(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes()).is_ok());

        assert_eq!(aggregator.clients.len(), 1);
        assert_ne!(aggregator.clients.get(&client_hash).unwrap().len(), 0);
    }

    #[test]
    fn expect_correctly_erase_data() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        assert!(aggregator.add_client(client_hash).is_ok());
        assert!(aggregator.read(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes()).is_ok());

        assert_eq!(aggregator.clients.len(), 1);
        assert_ne!(aggregator.clients.get(&client_hash).unwrap().len(), 0);

        assert!(aggregator.erase_data(client_hash).is_ok());
        assert_eq!(aggregator.clients.len(), 1);
        assert_eq!(aggregator.clients.get(&client_hash).unwrap().len(), 0);
    }


    #[test]
    fn expect_err_on_read_to_non_existing_client() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;

        let reading_result = aggregator.read(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes());
        assert!(reading_result.is_err());
        assert_eq!(reading_result.unwrap_err(), 
        AggregatorError{
            kind: AggregatorErrorKind::ClientNotExist,
            context: AggregatorErrorContext{
                context: AggregationContext::AggregationError,
                user: client_hash
            }
        })
    }

    #[test]
    fn expect_correctly_identify_status() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        assert!(aggregator.add_client(client_hash).is_ok());
        assert!(aggregator.read(client_hash, format!("Hello from the user: {}", client_hash).as_bytes()).is_ok());

        assert_eq!(aggregator.identify_status(client_hash).unwrap(), Ended::NotEnded);


        let client_hash: u64 = 1u64;
        assert!(aggregator.add_client(client_hash).is_ok());
        assert!(aggregator.read(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes()).is_ok());

        assert_eq!(aggregator.identify_status(client_hash).unwrap(), Ended::Ended);
    }

    #[test]
    fn expect_correctly_identify_status_while_reading_data() {
        let mut aggregator = Aggregator::new();
        let client_hash: u64 = 0u64;
        assert!(aggregator.add_client(client_hash).is_ok());
        let status = aggregator.read_with_status(client_hash, format!("Hello from the user: {}", client_hash).as_bytes()).unwrap();

        assert_eq!(status, Ended::NotEnded);


        let client_hash: u64 = 1u64;
        assert!(aggregator.add_client(client_hash).is_ok());
        let status = aggregator.read_with_status(client_hash, format!("Hello from the user: {}\r", client_hash).as_bytes()).unwrap();

        assert_eq!(status, Ended::Ended);
    }
}