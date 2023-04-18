use std::{sync::{Arc, RwLock}, collections::HashMap};
use nng::Socket;
use net_core::{transport::sockets::{Handler, Receiver, Sender}, jsons::{json_pcap_parser::JsonPcapParser, json_parser::JsonParser}};
use net_core::transport::connector_nng::Proto;
use serde_with::serde_as;

pub struct CommandDispatcher{ 
    pub queries: Arc<RwLock<HashMap<String, String>>>,
    pub connector: Arc<RwLock<Socket>>
}
pub struct CommandDispatcherBuilder {
    queries: HashMap<String, String>,
    end_point: String,
    proto: Proto
}
// probably this data structure won't be used further because of using middleware format
#[serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PacketData {
    #[serde_as(as = "serde_with::TimestampMilliSeconds<i64>")]
    pub frame_time: chrono::DateTime<chrono::Utc>,
    pub src_addr: String,
    pub dst_addr: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub json: serde_json::Value,
}

impl CommandDispatcherBuilder {
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.end_point = endpoint;
        self
    }
    pub fn with_proto(mut self, proto: Proto) -> Self {
        self.proto = proto;
        self
    }
    pub fn with_query_service(mut self, query_service_id: &str, query_service_addresss: &str) -> Self {
        self.queries.insert(query_service_id.to_owned(), query_service_addresss.to_owned());
        self
    }
    pub fn build(self) -> CommandDispatcher {
        let connector = Socket::new(Proto::into(self.proto)).unwrap();
        // temp listen
        connector.listen(self.end_point.as_str()).unwrap();
        CommandDispatcher { 
            queries: Arc::new(RwLock::new(self.queries)),
            connector: Arc::new(RwLock::new(connector))
        }
    } 
}
impl CommandDispatcher {
    pub fn builder() -> CommandDispatcherBuilder {
        CommandDispatcherBuilder { 
            queries: HashMap::<String, String>::default(),
            end_point: String::default(),
            proto: Proto::Req
        } 
    }
}
impl Handler for CommandDispatcher {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        //=======================================================================================
        // TODO: This block has to be moved to translator 
        //TODO should be moved to the task CU-861mdndny
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
            frame_time, 
            src_addr: src_addr.unwrap(),
            dst_addr: dst_addr.unwrap(),
            json: serde_json::from_slice(binary_json.as_slice()).unwrap()
        };
        let temp_topic = "add_packet".as_bytes().to_owned();
        log::debug!("Topic: {:?}", temp_topic);
        let mut data = bincode::serialize(&frame_data).unwrap();
        // manually adding topic into at the beginning of the data. Ideally it has to already be in the data 
        data.splice(0..0, temp_topic); 
        log::debug!("Data with topic: {:?}", data);
        self.connector.try_write().unwrap().send(data.as_slice());
    }
}