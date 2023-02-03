use std::sync::{Arc, RwLock};
use net_core::capture::decoder_binary::JsonDecoder;
use net_core::translator::Decoder;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct DecoderCommand<S> {
    pub push: Arc<S>,
}

impl<S: Sender> Handler for DecoderCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        println!("received from agent {:?}", data);

        let json_result = JsonDecoder::decode(data);
        println!("decoded data {:?}", json_result);

        self.push.send(json_result.into_bytes())
    }
}