use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use log::debug;

use net_core::transport::sockets::{Handler, Receiver, Sender};

use net_proto_api::decoder_api::Decoder;

use net_timescale_api::api::network_packet::NetworkPacketDTO;

use simple_websockets::{Message, Responder};

pub struct PullCommand {
    pub clients: Arc<RwLock<HashMap<u64, Responder>>>
}

impl Handler for PullCommand {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let network_packet_data = NetworkPacketDTO::decode(data);

        let formated_string = format!("{:?}", network_packet_data);

        debug!("received from server {:?}", formated_string);

        self.clients.read().unwrap().iter().for_each(|endpoint| {
            debug!("Connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Text(format!("{:?}", formated_string)));
        });
    }
}