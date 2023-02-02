use std::sync::{Arc, RwLock};
use net_core::capture::decoder_binary::BinaryDecoder;
use net_core::translator::Decoder;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct HubCommand;

impl Handler for HubCommand {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        println!("received from agent {:?}", data);

        let json_result = BinaryDecoder::decode(data);
        println!("decoded data {:?}", json_result)
    }
}