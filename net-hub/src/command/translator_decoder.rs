use std::sync::Arc;
use net_core::transport::sockets::{Sender, Handler, Receiver};


struct TranslatorDecoderCommand<S>
where S: Sender + ?Sized
{
    producer: Arc<S>
}

impl<S> Handler for TranslatorDecoderCommand<S>
where S: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        log::info!("In TranslatorDecoderCommand");
        self.producer.send(receiver.recv());
    }
}
