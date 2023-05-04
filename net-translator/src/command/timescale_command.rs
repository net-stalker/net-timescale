use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_core::topic::{remove_topic, DB_TOPIC};
pub struct TimescaleCommand<S>
where S: Sender + ?Sized
{
    pub consumer: Arc<S> 
}

impl<S> Handler for TimescaleCommand<S>
where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let mut data = receiver.recv();
        log::info!("received from TranslatorDispatcher {:?}", data);
        data = remove_topic(data, DB_TOPIC.as_bytes());
        self.consumer.send(data);
    }
}