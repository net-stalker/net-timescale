use std::{sync::{Arc, RwLock}, collections::HashMap};
use nng::Socket;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_core::transport::connector_nng::Proto;



pub struct NewDispatcher{
    pub queries: Arc<RwLock<HashMap<String, String>>>,
    pub connector: Arc<RwLock<Socket>>
}
pub struct NewDispatcherBuilder {
    queries: HashMap<String, String>,
    end_point: String,
    proto: Proto
}
impl NewDispatcherBuilder {
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
    pub fn build(self) -> NewDispatcher {
        let connector = Socket::new(Proto::into(self.proto)).unwrap();
        NewDispatcher { 
            queries: Arc::new(RwLock::new(self.queries)),
            connector: Arc::new(RwLock::new(connector))
        }
    } 
}
impl NewDispatcher {
    pub fn builder() -> NewDispatcherBuilder {
        NewDispatcherBuilder { 
            queries: HashMap::<String, String>::default(),
            end_point: String::default(),
            proto: Proto::Req
        } 
    }
}
impl Handler for NewDispatcher {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        // Here I should get data and retransmit it. 
        let data = receiver.recv();
        log::info!("Handle in S_Dispatcher is triggered");   
        if let Some((key, value)) = self.queries.try_read().unwrap().get_key_value("1") {
            let mut con = self.connector.try_write().unwrap();
            con.dial(value).unwrap();
            con.send(*b"Hello from dispatcher").unwrap();
        } else {
            log::error!("Map is empty!");
        }  
    }
}