use std::sync::{Arc, RwLock};
use nng::Socket;
use net_core::{transport::sockets::{Handler, Receiver, Sender}, jsons::{json_pcap_parser::JsonPcapParser, json_parser::JsonParser}};
use net_core::transport::connector_nng::Proto;

use crate::db_access::add_traffic::packet_data::PacketData;

pub struct CommandDispatcher{ 
    pub connector: Arc<RwLock<Socket>>,
    end_point: String
}
pub struct CommandDispatcherBuilder {
    end_point: String,
    proto: Proto
}

// type CommandDispatcherBuilder = PubSubConnectorNngBuilder 

impl CommandDispatcherBuilder {
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.end_point = endpoint;
        self
    }
    pub fn with_proto(mut self, proto: Proto) -> Self {
        self.proto = proto;
        self
    }
    pub fn build(self) -> CommandDispatcher {
        CommandDispatcher { 
            connector: Arc::new(RwLock::new(Socket::new(Proto::into(self.proto)).unwrap())),
            end_point: self.end_point
        }
    } 
}
impl CommandDispatcher {
    pub fn builder() -> CommandDispatcherBuilder {
        CommandDispatcherBuilder { 
            end_point: String::default(),
            proto: Proto::Pub
        } 
    }
    pub fn bind(self) -> Self {
        self.connector.try_read().unwrap().listen(self.end_point.as_str()).unwrap();
        self
    }
}
impl Handler for CommandDispatcher {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        
        /*
        --------------------------
        CAPNPROTO PLAYGROUND START
        --------------------------
        */

        self.connector.try_write().unwrap().send(&data).unwrap();

        /*
        ------------------------
        CAPNPROTO PLAYGROUND END
        ------------------------
        */
    }
}