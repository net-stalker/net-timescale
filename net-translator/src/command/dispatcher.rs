use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};

// TODO: think about creating a common Dispatcher struct and move it to net-core
pub struct TranslatorDispatcher<T>
where T: Sender + ?Sized
{
    pub producer: Arc<T>
}

impl<T> Handler for TranslatorDispatcher<T>
where T: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        log::info!("Dispatcher in translator");
        self.producer.send(receiver.recv());
    }
}