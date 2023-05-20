use std::sync::Arc;
use net_core::{transport::sockets::{Handler, Receiver, Sender}, topic::set_topic};


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
        log::info!("received from hub {}", envelope.get_type());
        self.consumer.send(set_topic(envelope.get_data().to_owned(), envelope.get_type().as_bytes()));
    }
}