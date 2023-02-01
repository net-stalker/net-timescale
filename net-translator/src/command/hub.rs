use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct HubCommand;

impl Handler for HubCommand {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        println!("received from agent {:?}", data);
    }
}