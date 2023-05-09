use super::sockets::{Handler, Receiver, Sender};

pub struct DummyCommand;

impl Handler for DummyCommand {
    fn handle(&self, _receiver: &dyn Receiver, _sender: &dyn Sender) {}
}