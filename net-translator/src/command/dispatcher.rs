use std::sync::Arc;
use net_core::{transport::sockets::{Handler, Receiver, Sender}, topic::set_topic};
use net_proto_api::{envelope::envelope::Envelope, decoder_api::Decoder};

pub struct TranslatorDispatcher<T>
where T: Sender + ?Sized
{
    pub consumer: Arc<T>
}

impl<T> Handler for TranslatorDispatcher<T>
where T: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("received data from hub: {:?}", data);
        let message = Envelope::decode(data);
        let mut buffer = message.get_data().to_owned();
        buffer = set_topic(buffer, message.get_type().as_bytes());
        self.consumer.send(message.get_data().to_owned());
    }
}