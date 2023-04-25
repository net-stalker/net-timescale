use std::sync::Arc;

use log::debug;
use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;
use net_core::jsons::json_parser::JsonParser;
use net_core::jsons::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};

use crate::capnp::data_to_send_capnp;

pub struct DecoderCommand<S> {
    pub push: Arc<S>,
}

impl<S: Sender> Handler for DecoderCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        debug!("received from agent {:?}", data);
         

        /*
        --------------------------
        CAPNPROTO PLAYGROUND START
        --------------------------
        */


        let json_bytes = PcapTranslator::translate(data);

        let filtered_value_json = JsonPcapParser::filter_source_layer(&json_bytes);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);

        let frame_time = JsonPcapParser::find_frame_time(&json_bytes);
        let src_addr = JsonPcapParser::extract_src_addr_l3(&layered_json);
        let dst_addr = JsonPcapParser::extract_src_addr_l3(&layered_json);
        let binary_json = JsonParser::get_vec(layered_json);

        // debug!("{:?} {:?} {:?} {:?}", frame_time, src_addr, dst_addr, binary_json);

        let mut buffer: Vec<u8> = Vec::new();
        
        let packed_data = crate::capnp::data_to_send::form_data(
            &mut buffer,
            frame_time.timestamp_millis(), 
            src_addr.unwrap(), 
            dst_addr.unwrap(), 
            binary_json);

        
        //self.push.send(packed_data);

        /*
        ------------------------
        CAPNPROTO PLAYGROUND END
        ------------------------
        */


        // self.push.send(binary_json)

        self.push.send(buffer)
    }
}