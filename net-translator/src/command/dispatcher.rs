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
        let message = Envelope::decode(data);
        log::info!("received data type from hub {}", message.get_type());
        self.consumer.send(set_topic(message.get_data().to_owned(), message.get_type().as_bytes()));
    }
}