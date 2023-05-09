use std::sync::Arc;

use log::debug;

use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;
use net_core::jsons::json_parser::JsonParser;
use net_core::jsons::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};

use net_timescale_api::{self, Encoder};
use net_timescale_api::api::envelope::Envelope;
use net_timescale_api::api::network_packet::NetworkPacket;
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
        let data = receiver.recv();
        debug!("received from translator::dispatcher {:?}", data);

        let json_bytes = PcapTranslator::translate(data);

        let filtered_value_json = JsonPcapParser::filter_source_layer(&json_bytes);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);

        let frame_time = JsonPcapParser::find_frame_time(&json_bytes);
        let src_addr = JsonPcapParser::extract_src_addr_l3(&layered_json);
        let dst_addr = JsonPcapParser::extract_src_addr_l3(&layered_json);
        let binary_json = JsonParser::get_vec(layered_json);

        
        let net_packet = NetworkPacket::new(
            frame_time.timestamp_millis(), 
            src_addr.unwrap(), 
            dst_addr.unwrap(), 
            binary_json);
            
        let envelope = Envelope::new(
            String::from("add_packet"),
            net_packet.encode()
        );
        
        let message: Vec<u8> = envelope.encode();

        self.transmitter.send(message);
    }
}