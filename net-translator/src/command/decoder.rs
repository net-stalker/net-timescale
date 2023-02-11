use std::sync::{Arc};

use net_core::capture::decoder_binary::PcapTranslator;
use net_core::capture::translator::layer_extractor::LayerExtractor;
use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;
use net_core::translator::Translator;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct DecoderCommand<S> {
    pub push: Arc<S>,
}

impl<S: Sender> Handler for DecoderCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        println!("received from agent {:?}", data);

        let json_as_bytes = PcapTranslator::translate(data);
        let json = LayerExtractor::translate(json_as_bytes);
        println!("decoded data {:?}", json);

        self.push.send(json_as_bytes.into_bytes())
    }
}