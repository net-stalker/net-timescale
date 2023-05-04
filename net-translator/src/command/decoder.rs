use std::sync::Arc;

use log::debug;
use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_core::topic::{remove_topic, set_topic, DECODER_TOPIC, DB_TOPIC};

pub struct DecoderCommand<S>
where S: Sender + ?Sized
{
    pub transmitter: Arc<S>,
}

impl<S> Handler for DecoderCommand<S>
where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let mut data = receiver.recv();
        debug!("received from translator::dispatcher {:?}", data);

        // let filtered_value_json = JsonPcapParser::filter_source_layer(&json_bytes);
        // let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        // let layered_json = JsonPcapParser::split_into_layers(first_json_value);

        // let frame_time = JsonPcapParser::find_frame_time(&json_bytes);
        // let src_addr = JsonPcapParser::extract_src_addr_l3(&layered_json);
        // let dst_addr = JsonPcapParser::extract_src_addr_l3(&layered_json);
        // let binary_json = JsonParser::get_vec(layered_json);

        // debug!("{:?} {:?} {:?} {:?}", frame_time, src_addr, dst_addr, binary_json);

        // self.push.send(binary_json)
        // self.push.send(json_bytes)

        //========================
        // TODO: remove this part in future
        data = remove_topic(data, DECODER_TOPIC.as_bytes());

        let mut json_bytes = PcapTranslator::translate(data);

        json_bytes = set_topic(json_bytes, DB_TOPIC.as_bytes());
        //========================

        self.transmitter.send(json_bytes);
    }
}