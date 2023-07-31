use std::sync::Arc;

use net_transport::sockets::{Sender, Receiver, Handler};

pub struct Router<T>
where T: Sender + Sized
{
    network_channel: Arc<T>
}
impl<T> Router<T>
where T: Sender + Sized 
{
    pub fn new(network_channel: Arc<T>) -> Self {
        Router { network_channel }
    } 
}
impl<T> Handler for Router<T>
where T: Sender + Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        self.network_channel.send(receiver.recv().as_slice());
    }
}