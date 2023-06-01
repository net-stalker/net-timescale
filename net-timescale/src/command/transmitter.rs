use std::sync::Arc;

use net_core::transport::sockets::{Sender, Receiver, Handler};

pub struct Transmitter<T>
where T: Sender + Sized
{
    network_channel: Arc<T>
}
impl<T> Transmitter<T>
where T: Sender + Sized 
{
    pub fn new(network_channel: Arc<T>) -> Self {
        Transmitter { network_channel } 
    } 
}
impl<T> Handler for Transmitter<T>
where T: Sender + Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        self.network_channel.send(receiver.recv().as_slice());
    }
}