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

        self.connector.try_write().unwrap().send(&data).unwrap();
    }
}