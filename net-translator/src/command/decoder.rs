use std::sync::Arc;

use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;
use net_core::json_parser::JsonParser;
use net_core::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct DecoderCommand<S> {
    pub push: Arc<S>,
}

impl<S: Sender> Handler for DecoderCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        println!("received from agent {:?}", data);

        let json_as_bytes = PcapTranslator::translate(data);
        let result = JsonPcapParser::find_source_layer(json_as_bytes);
        let json_vec = JsonParser::get_vec(result.0);

        self.push.send(json_vec)
    }
}