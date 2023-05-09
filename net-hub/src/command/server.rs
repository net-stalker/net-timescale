use std::sync::Arc;
use log::debug;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_core::topic::{check_topic, DB_TOPIC, DECODER_TOPIC, set_topic, remove_topic};
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
        if check_topic(&data, DECODER_TOPIC.as_bytes()) {
            debug!("received from agent {:?}", data);
        } else {
            debug!("received from decoder {:?}", data);
            let clients_data = data.clone();
            self.clients.send(clients_data);
            data = set_topic(data, DB_TOPIC.as_bytes());
        }
        self.translator.send(data);
    }
}