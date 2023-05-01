use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct DummyHandler;

impl Handler for DummyHandler {
    fn handle(&self, _receiver: &dyn Receiver, _sender: &dyn Sender) {}
}