use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct DummyTimescaleHandler;

impl Handler for DummyTimescaleHandler {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("received from timescale: {}", String::from_utf8(data).unwrap());
    }
}