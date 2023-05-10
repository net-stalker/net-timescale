use std::sync::Arc;
use net_core::{transport::sockets::{Handler, Receiver, Sender}};


use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::decoder_api::Decoder;

pub struct CommandDispatcher<T>
where T: Sender + ?Sized
{ 
    consumer: Arc<T>,
}
impl<T> CommandDispatcher<T>
where T: Sender + ?Sized
{
    pub fn new(consumer: Arc<T>) -> Self {
        CommandDispatcher { consumer }
    }
}
impl<T> Handler for CommandDispatcher<T>
where T: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(data);
        let mut data = envelope.get_data().to_owned();
        // TODO: think about adding HashMap in dispatcher with connectors to avoid such overheads
        data.splice(0..0, envelope.get_type().as_bytes().to_owned());
        self.consumer.send(data);
    }
}