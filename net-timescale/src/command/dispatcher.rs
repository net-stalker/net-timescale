use std::rc::Rc;

use net_transport::{
    sockets::{Handler, Receiver, Sender, Pub},
};

use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;

pub struct CommandDispatcher<T>
where T: Sender + Pub + ?Sized
{ 
    consumer: Rc<T>,
}
impl<T> CommandDispatcher<T>
where T: Sender + Pub + ?Sized
{
    pub fn new(consumer: Rc<T>) -> Self {
        CommandDispatcher { consumer }
    }
}
impl<T> Handler for CommandDispatcher<T>
where T: Sender + Pub + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(&data);
        self.consumer.set_topic(envelope.get_type().as_bytes());
        self.consumer.send(envelope.encode().as_slice());
    }
}