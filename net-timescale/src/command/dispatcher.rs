use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use net_core::jsons::json_parser::JsonParser;
use net_core::jsons::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};

use bincode;
use serde::{Serialize, Deserialize};
use crate::query::request::Request;

pub struct CommandDispatcher {
    pub queries: Arc<RwLock<HashMap<String, Box<dyn Request>>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FrameData {
    pub frame_time: String,
    pub src_addr: String,
    pub dst_addr: String,
    pub binary_json: Vec<u8>,
}

impl Handler for CommandDispatcher {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();

        //TODO should be moved to the task CU-861mdndny
        let filtered_value_json = JsonPcapParser::filter_source_layer(&data);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);
        // TODO: think about avoiding converting frame_time into DateTime<Local> at once because it can't be serialized  
        let frame_time = JsonPcapParser::find_frame_time(&data);
        let src_addr = JsonPcapParser::extract_src_addr_l3(&layered_json)
            .or(Some("".to_string()));
        let dst_addr = JsonPcapParser::extract_dst_addr_l3(&layered_json)
            .or(Some("".to_string()));
        let binary_json = JsonParser::get_vec(layered_json);
        
        // TODO: move this part into a separate `ParametersBuilder`
        let frame_data = FrameData {
            frame_time: frame_time.to_string(),
            src_addr: src_addr.unwrap(),
            dst_addr: dst_addr.unwrap(),
            binary_json,
        };
        
        let buffer = bincode::serialize(&frame_data).unwrap();
        // query result can be sent back
        // depends on query type
        // unused just for now
        let _result = self.queries.read().unwrap()
            .get("insert_packet").unwrap()
            .execute(buffer);
    }
}