use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct ClientCommand;

impl Handler for ClientCommand {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {}
}