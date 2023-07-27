use std::cell::RefCell;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use websocket::{ClientBuilder, Message};
use websocket::sync::Client;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;

pub struct WsRouter {
    pub ws: RefCell<Client<TcpStream>>
}

impl WsRouter {
    pub fn new(end_point: &str) -> Self {
        let mut client = match ClientBuilder::new(end_point)
            .unwrap()
            .connect_insecure()
        {
            Ok(con) => {con},
            Err(err) => {
                log::error!("{}", err);
                panic!();
            }
        };
        Self {
            ws: RefCell::new(client),
        }
    }

    pub fn send(&self, data: Vec<u8>) {
        let message = Message::binary(data);
        self.ws.borrow_mut().send_message(&message).unwrap();
    }
}

impl Handler for WsRouter {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(data.as_slice());
        log::debug!("msg {} in ws_router", envelope.get_type());
        self.send(data);
    }
}