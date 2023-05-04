use std::sync::Arc;
use net_core::{transport::sockets::{Handler, Receiver, Sender}, jsons::{json_pcap_parser::JsonPcapParser, json_parser::JsonParser}};

use crate::db_access::add_traffic::packet_data::PacketData;

pub struct CommandDispatcher<T>
where T: Sender + ?Sized
{ 
    consumer: Arc<T>,
}
impl<T> CommandDispatcher<T>
where T: Sender + ?Sized
{
    pub fn new(consumer: Arc<T>) -> Self {
        CommandDispatcher { consumer }
    }
}
impl<T> Handler for CommandDispatcher<T>
where T: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("Got data in dispatcher");
        // //=======================================================================================
        // TODO: This block has to be moved to translator 
        // TODO: should be moved to the task CU-861mdndny
        let filtered_value_json = JsonPcapParser::filter_source_layer(&data);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);  
        let frame_time = JsonPcapParser::find_frame_time(&data);
        let src_addr = JsonPcapParser::extract_src_addr_l3(&layered_json)
            .or(Some("".to_string()));
        let dst_addr = JsonPcapParser::extract_dst_addr_l3(&layered_json)
            .or(Some("".to_string()));
        let binary_json = JsonParser::get_vec(layered_json);
        
        let frame_data = PacketData {
            frame_time: frame_time.timestamp_millis(), 
            src_addr: src_addr.unwrap(),
            dst_addr: dst_addr.unwrap(),
            binary_json
        };
        let temp_topic = "add_packet".as_bytes().to_owned();
        let mut data = bincode::serialize(&frame_data).unwrap();
        // manually adding topic into at the beginning of the data. Ideally it has to already be in the data 
        data.splice(0..0, temp_topic); 
        self.consumer.send(data);
    }
}