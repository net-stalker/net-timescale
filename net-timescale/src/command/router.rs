use std::rc::Rc;
use net_transport::sockets::{Sender, Receiver, Handler};

pub struct Router<T>
where T: Sender + Sized
{
    network_channel: Rc<T>
}
impl<T> Router<T>
where T: Sender + Sized 
{
    pub fn new(network_channel: Rc<T>) -> Self {
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