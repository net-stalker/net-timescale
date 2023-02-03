use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use simple_websockets::{Message, Responder};
use net_core::transport::sockets::{Handler, Receiver, Sender};
use serde_json::json;
use unescape::unescape;

pub struct PullCommand {
    pub clients: Arc<RwLock<HashMap<u64, Responder>>>,
}

impl Handler for PullCommand {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        let data = receiver.recv();
        let string_with_escapes = String::from_utf8(data).unwrap();
        // let unescaped_string = unescape(string_with_escapes.as_str()).unwrap();
        // let json_string = json!(&unescaped_string);
        // println!("string with escapes: {}", string_with_escapes);
        // println!("string without escapes: {}", unescaped_string);
        // println!("json: {}", json_string);
        println!("received from translator {:?}", string_with_escapes);

        // let magic_num = &data[..4];
        // if 3569595041_u32.to_be_bytes() == magic_num {
        // println!("Global header will be skipped");
        // return;
        // }

        self.clients.read().unwrap().iter().for_each(|endpoint| {
            println!("Connections: {:?}", endpoint);
            let responder = endpoint.1;
            responder.send(Message::Text(format!("{:?}", string_with_escapes)));
        });
    }
}