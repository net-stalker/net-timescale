use std::sync::Arc;
use net_core::transport::sockets::{Sender, Receiver, Handler, Socket};

pub struct ResultSender {
    pub connector: Arc<dyn Socket>
}
impl Handler for ResultSender {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        self.connector.get_sender().send(data);
    }
}