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
        let data = receiver.recv();

        // let magic_num = &data[..4];
        // if 3569595041_u32.to_be_bytes() == magic_num {
        // debug!("Global header will be skipped");
        // return;
        // }

        // just for logging
        if check_topic(&data, DB_TOPIC.as_bytes()) {
            debug!("received from decoder {:?}", data);
            let mut clients_data = data.clone();
            clients_data = remove_topic(clients_data, DB_TOPIC.as_bytes());
            self.clients.send(clients_data);
        } else {
            debug!("received from agent {:?}", data);
        }
        self.translator.send(data);
    }
}