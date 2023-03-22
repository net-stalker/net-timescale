use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use net_core::jsons::json_parser::JsonParser;
use net_core::jsons::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};

use crate::query::insert_packet::InsertPacket;

use super::connection_pool::ConnectionPool;

pub struct CommandDispatcher {
    pub queries: Arc<RwLock<HashMap<String, InsertPacket>>>,
    pub connections: ConnectionPool
}

impl Handler for CommandDispatcher {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();

        //TODO should be moved to the task CU-861mdndny
        let filtered_value_json = JsonPcapParser::filter_source_layer(&data);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);

        let frame_time = JsonPcapParser::find_frame_time(&data);
        let mut src_addr = JsonPcapParser::extract_src_addr_l3(&layered_json)
            .or(Some("".to_string()));
        let mut dst_addr = JsonPcapParser::extract_dst_addr_l3(&layered_json)
            .or(Some("".to_string()));
        let binary_json = JsonParser::get_vec(layered_json);

        self.queries.read().unwrap()
            .get("insert_packet").unwrap()
            .insert(frame_time, src_addr.unwrap(), dst_addr.unwrap(), binary_json);
    }
}