use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct ClientCommand;

impl Handler for ClientCommand {
    fn handle(&self, _receiver: &dyn Receiver, _sender: &dyn Sender) {}
}