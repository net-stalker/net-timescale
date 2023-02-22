use std::sync::Arc;

use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;
use net_core::jsons::json_parser::JsonParser;
use net_core::jsons::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct DecoderCommand<S> {
    pub push: Arc<S>,
}

impl<S: Sender> Handler for DecoderCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        println!("received from agent {:?}", data);

        let json_bytes = PcapTranslator::translate(data);
        let filtered_value_json = JsonPcapParser::filter_source_layer(json_bytes);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let splited_json = JsonPcapParser::split_into_layers(first_json_value);
        let json_vec = JsonParser::get_vec(splited_json);

        self.push.send(json_vec)
    }
}