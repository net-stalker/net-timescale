use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::serde::ts_milliseconds;
use net_core::jsons::json_parser::JsonParser;
use net_core::jsons::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use crate::db_access;
// TODO: dispatcher has to be redesigned 
pub struct CommandDispatcher<H>
where
    H: db_access::as_query::AsQuery + ?Sized
{
    pub queries: Arc<RwLock<HashMap<String, Box<H>>>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PacketData {
    #[serde(with = "ts_milliseconds")]
    pub frame_time: chrono::DateTime<chrono::Utc>,
    pub src_addr: String,
    pub dst_addr: String,
    pub binary_json: Vec<u8>,
}
// receiver sends serizalized data. Then this data 
impl<H> Handler for CommandDispatcher<H>
where 
    H: db_access::as_query::AsQuery + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        //=======================================================================================
        // TODO: This block has to be moved to translator 
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
        
        let frame_data = PacketData {
            frame_time: frame_time.into(), 
            src_addr: src_addr.unwrap(),
            dst_addr: dst_addr.unwrap(),
            // `bincode` doesn't know how to serialize serde_json::Value. 
            // TODO: investigate serializing serde_json::Value to avoid avoid this inconvenience
            // json: serde_json::from_slice(binary_json.as_slice()).unwrap(), - produces a runtime error
            binary_json
        };

        let data = bincode::serialize(&frame_data).unwrap();
        //=====================================================================================
        let _result = self.queries.read().unwrap()
            .get("insert_packet").unwrap()
            .execute(data.as_slice());
    }
}