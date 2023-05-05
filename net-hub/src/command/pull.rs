use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use log::debug;
use net_timescale_api::Decoder;
use net_timescale_api::api::network_packet::NetworkPacket;
use net_timescale_api::api::envelope::Envelope;

use simple_websockets::{Message, Responder};

use net_core::transport::sockets::{Handler, Receiver, Sender};


pub struct PullCommand<S> {
    pub clients: Arc<RwLock<HashMap<u64, Responder>>>,
    pub db_service: Arc<S>,
}

impl<S: Sender> Handler for PullCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();

        let envelope = Envelope::decode(data.clone());
        let network_packet_data = NetworkPacket::decode(envelope.get_data().to_owned());

        let formated_string = format!("{:?}", network_packet_data);

        // let unescaped_string = unescape(string_with_escapes.as_str()).unwrap();
        // let json_string = json!(&unescaped_string);
        // debug!("string with escapes: {}", string_with_escapes);
        // debug!("string without escapes: {}", unescaped_string);
        // debug!("json: {}", json_string);
        debug!("received from translator {:?}", formated_string);

        self.clients.read().unwrap().iter().for_each(|endpoint| {
            debug!("Connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Text(format!("{:?}", formated_string)));
        });

        self.db_service.send(data);
    }
}