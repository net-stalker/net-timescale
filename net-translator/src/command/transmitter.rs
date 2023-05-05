use std::sync::Arc;
use net_core::{
    transport::{
        sockets::{Handler, Receiver, Sender}
    },
};

pub struct Transmitter<S>
where S: Sender + ?Sized
{
    pub consumer: Arc<S>
}

impl<S> Handler for Transmitter<S>
where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        self.consumer.send(data);
    }
}