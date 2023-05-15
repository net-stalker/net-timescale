use std::sync::Arc;
use log::debug;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_core::topic::{check_topic, DB_TOPIC, DECODER_TOPIC, set_topic, remove_topic};
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
pub struct ServerCommand<S: ?Sized> {
    pub translator: Arc<S>,
    pub clients: Arc<S>
}

impl<S: Sender + ?Sized> Handler for ServerCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let mut data = receiver.recv();

        // let magic_num = &data[..4];
        // if 3569595041_u32.to_be_bytes() == magic_num {
        // debug!("Global header will be skipped");
        // return;
        // }
        let mut message = Envelope::decode(data);

        if message.get_type() == DECODER_TOPIC {
            debug!("received from agent {}", message.get_type());
        } else {
            debug!("received from decoder {}", message.get_type());
            self.clients.send(message.get_data().to_owned());
            message = Envelope::new(DB_TOPIC.to_owned(), message.encode());
        }
        data = message.encode();
        
        self.translator.send(data);
    }
}