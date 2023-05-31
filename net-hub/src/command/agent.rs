use std::sync::Arc;
use log::debug;
use net_core::transport::sockets::{Handler, Receiver, Sender};

pub struct AgentCommand<S: ?Sized> {
    pub translator: Arc<S>
}

impl<S: Sender + ?Sized> Handler for AgentCommand<S> {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        
        let data = receiver.recv();
        debug!("received data from net-agent");
        self.translator.send(data);

        // let magic_num = &data[..4];
        // if 3569595041_u32.to_be_bytes() == magic_num {
        // debug!("Global header will be skipped");
        // return;
        // }
    }
}