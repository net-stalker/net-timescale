use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use simple_websockets::{Message, Responder};
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct AgentCommand {
    pub clients: Arc<RwLock<HashMap<u64, Responder>>>,
}

impl Handler for AgentCommand {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        let data = receiver.recv();

        println!("received from agent {:?}", data);
        let magic_num = &data[..4];
        if 3569595041_u32.to_be_bytes() == magic_num {
            println!("Global header will be skipped");
            return;
        }

        self.clients.read().unwrap().iter().for_each(|endpoint| {
            println!("Connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Text(format!("{:?}", &data)));
        });
    }
}