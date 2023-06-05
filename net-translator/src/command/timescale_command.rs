use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};
pub struct TimescaleCommand<S>
where S: Sender + ?Sized
{
    pub consumer: Arc<S> 
}

impl<S> Handler for TimescaleCommand<S>
where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::debug!("received from decoder");
        self.consumer.send(data.as_slice());
    }
}