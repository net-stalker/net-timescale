use std::sync::Arc;
use net_core::transport::sockets::{Sender, Receiver, Handler, Socket};

// Construct ResultSender
pub struct ResultSender {
    pub connector: Arc<dyn Socket>
}
// use handler to send data back from known endpoint
impl Handler for ResultSender {
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        let data = receiver.recv();
        self.connector.get_sender().send(data);
    }
}