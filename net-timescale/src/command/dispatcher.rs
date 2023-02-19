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
        let result = JsonPcapParser::find_frame_time(json_binary);

        self.queries.read().unwrap()
            .get("insert_packet").unwrap()
            .insert(result.0, result.1);
    }
}