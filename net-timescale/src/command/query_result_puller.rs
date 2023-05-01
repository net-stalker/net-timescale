use std::sync::Arc;

use net_core::transport::sockets::{Sender, Receiver, Handler};

pub struct QueryResultPuller<T>
where T: Sender + Sized
{
    pub network_channel: Arc<T>
}
impl<T> QueryResultPuller<T>
where T: Sender + Sized 
{
    pub fn new(network_channel: Arc<T>) -> Self {
        QueryResultPuller { network_channel } 
    } 
}
impl<T> Handler for QueryResultPuller<T>
where T: Sender + Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        self.network_channel.send(receiver.recv());
    }
}