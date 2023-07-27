use std::cell::RefCell;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use simple_websockets::Message;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use crate::command::ws_context::WsContext;

pub struct WsRouter {
    context: WsContext,
}

impl WsRouter {
    pub fn new(context: WsContext) -> Self {
        Self {
            context,
        }
    }

    pub fn send_broadcast(&self, data: Vec<u8>) {
        let ws_connections = self.context.get_connections().unwrap();
        ws_connections.iter().for_each(|connection| {
            connection.1.send(Message::Binary(data.clone()));
        });
    }

    pub fn send_to_connection(&self, connection_id: u64, data: Vec<u8>) {
        let ws_connection = self.context.get_connection_by_id(connection_id).unwrap();
        ws_connection.send(Message::Binary(data));
    }
}

impl Handler for WsRouter {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(data.as_slice());
        log::debug!("msg {} in ws_router", envelope.get_type());
        match envelope.get_type() {
            "network_graph" => {
                self.send_broadcast(data);
            },
            _ => ()
        }
    }
}