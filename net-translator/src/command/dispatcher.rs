use std::sync::Arc;
use log::debug;
use net_core::{transport::sockets::{Handler, Receiver, Sender}};

pub struct TranslatorDispatcher<T>
    where T: Sender + ?Sized
{
    pub consumer: Arc<T>,
}

impl<T> Handler for TranslatorDispatcher<T>
    where T: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        debug!("received data from agent-gateway");
        let data = receiver.recv();
        self.consumer.send(data.as_slice());
    }
}