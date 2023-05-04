use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct TranslatorDispatcher<T>
where T: Sender + ?Sized
{
    pub consumer: Arc<T>
}

impl<T> Handler for TranslatorDispatcher<T>
where T: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("received data from hub: {:?}", data);
        self.consumer.send(data);
    }
}