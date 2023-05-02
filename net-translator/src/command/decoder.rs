use std::sync::Arc;

use log::debug;
use net_core::capture::translator::pcap_translator::PcapTranslator;
use net_core::capture::translator::translator::Translator;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_core::transport::connector_nng::{ConnectorNNG, Proto};

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
        // remove this part in future
        let temp_topic = "decode".as_bytes().to_owned();
        let data = data[temp_topic.len()..].to_owned();

        let mut json_bytes = PcapTranslator::translate(data);

        let temp_topic = "db".as_bytes().to_owned();
        json_bytes.splice(0..0, temp_topic);
        //========================

        self.transmitter.send(json_bytes);
    }
}


#[cfg(test)]
mod tests{
    #[test]
    fn test_topic_replacement() {
        let decode_topic = "decode".as_bytes().to_owned();
        let db_topic = "db".as_bytes().to_owned();
        let mut test_data: Vec<u8> = vec![1, 1, 1];
        let decode_topic_test: Vec<u8> = vec![100, 101, 99, 111, 100, 101, 1, 1, 1];
        let db_topic_test: Vec<u8> = vec![100, 98, 1, 1, 1];
        test_data.splice(0..0, decode_topic.clone());
        
        assert_eq!(test_data, decode_topic_test);
       
        let mut test_data = test_data[decode_topic.len()..].to_owned();
        test_data.splice(0..0, db_topic);

        assert_eq!(test_data, db_topic_test);
    }
}