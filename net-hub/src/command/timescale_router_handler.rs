use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct TimescaleRouter<S>
where S: Sender
{
    pub consumer: Arc<S>
}

impl<S> Handler for TimescaleRouter<S>
where S: Sender
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("received from timescale: {}", String::from_utf8(data.clone()).unwrap());
        self.consumer.send(data.as_slice());
    }
}