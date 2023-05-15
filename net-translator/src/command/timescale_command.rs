use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_core::topic::{remove_topic, DB_TOPIC};
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
pub struct TimescaleCommand<S>
where S: Sender + ?Sized
{
    pub consumer: Arc<S> 
}

impl<S> Handler for TimescaleCommand<S>
where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let message = Envelope::decode(receiver.recv());
        log::info!("received from TranslatorDispatcher {:?}", message);
        self.consumer.send(message.get_data().to_owned());
    }
}