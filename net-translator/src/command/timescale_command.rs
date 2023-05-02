use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct TimescaleCommand<S>
where S: Sender + ?Sized
{
    pub producer: Arc<S> 
}

impl<S> TimescaleCommand<S>
where S: Sender + ?Sized
{
    fn remove_topic(mut data: Vec<u8>) -> Vec<u8> {
        if data.len() < 2 {
            return data;
        }

        data = data[2..].to_owned();
        data
    }   
}

impl<S> Handler for TimescaleCommand<S>
where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let mut data = receiver.recv();
        data = TimescaleCommand::<S>::remove_topic(data);
        log::info!("In TimescaleCommand: {:?}", data);
        self.producer.send(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_topic() {
        let test_data = vec![1, 1, 1];
        let mut data = vec![100, 98, 1, 1, 1];
        data = TimescaleCommand::<dyn Sender>::remove_topic(data);
        assert_eq!(data, test_data);

        data = vec![1];
        let test_data = vec![1];
        let data = TimescaleCommand::<dyn Sender>::remove_topic(data);

        assert_eq!(data, test_data);
    }
}