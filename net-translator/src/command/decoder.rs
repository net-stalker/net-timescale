use std::sync::Arc;

use log::debug;

use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;

use net_core::jsons::json_parser::JsonParser;
use net_core::jsons::json_pcap_parser::JsonPcapParser;

use net_core::transport::sockets::{Handler, Receiver, Sender};

use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::encoder_api::Encoder;

use net_timescale_api::api::network_packet::NetworkPacketDTO;

pub struct DecoderCommand<S>
    where S: Sender + ?Sized
{
    pub consumer: Arc<S>,
}

impl<S> Handler for DecoderCommand<S>
    where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();

        let message = Envelope::decode(data);

        let _msg_type = message.get_type().to_owned();
        let data = message.get_data().to_owned();

        debug!("received msg type from: {}", message.get_type());

        let json_bytes = PcapTranslator::translate(data);

        let filtered_value_json = JsonPcapParser::filter_source_layer(&json_bytes);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);

        let frame_time = JsonPcapParser::find_frame_time(&json_bytes);
        let src_addr = match JsonPcapParser::extract_src_addr_l3(&layered_json) {
            Some(src) => src,
            None => {
                log::error!("src is missing");
                return;
            }
        };
        let dst_addr = match JsonPcapParser::extract_dst_addr_l3(&layered_json) {
            Some(dst) => dst,
            None => {
                log::error!("dst is missing");
                return;
            }
        };
        let binary_json = JsonParser::get_vec(layered_json);

        let net_packet = NetworkPacketDTO::new(
            frame_time.timestamp_millis(),
            src_addr,
            dst_addr,
            binary_json);

        let envelope = Envelope::new(
            "network_packet".to_owned(),
            net_packet.encode(),
        );

        let message: Vec<u8> = envelope.encode();
        // for now we don't set any topics because we have the only local service for this data to recieve
        // ideally we need to set here the same topic which has been received from net-hub
        self.consumer.send(message.as_slice());
    }
}