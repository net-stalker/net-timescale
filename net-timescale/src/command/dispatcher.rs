use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use net_core::json_pcap_parser::JsonPcapParser;
use net_core::transport::sockets::{Handler, Receiver, Sender};

use crate::query::insert_packet::InsertPacket;

pub struct CommandDispatcher {
    pub queries: Arc<RwLock<HashMap<String, InsertPacket>>>,
}

impl Handler for CommandDispatcher {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let json_binary = receiver.recv();
        let json_binary_clone = json_binary.clone();
        let frame_time = JsonPcapParser::find_frame_time(json_binary);

        self.queries.read().unwrap()
            .get("insert_packet").unwrap()
            .insert(frame_time, json_binary_clone);
    }
}