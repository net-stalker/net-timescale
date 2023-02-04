use std::sync::{Arc, RwLock};
use net_core::capture::decoder_binary::JsonDecoder;
use net_core::translator::Decoder;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct DummyCommand;

impl Handler for DummyCommand {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
    }
}