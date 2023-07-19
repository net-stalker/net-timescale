use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct Router<S>
where S: Sender
{
    pub consumer: Arc<S>
}

impl<S> Handler for Router<S>
where S: Sender
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("received data: {:?}", data);
        self.consumer.send(data.as_slice());
    }
}