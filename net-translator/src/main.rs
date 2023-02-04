use std::sync::{Arc, RwLock};
use std::thread;
use net_core::capture::decoder_binary::JsonDecoder;
use net_core::translator::Decoder;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;
use net_translator::command::decoder::DecoderCommand;
use net_translator::command::dummy::DummyCommand;

fn main() {
    let push = ConnectorNNG::builder()
        .with_endpoint("tcp://0.0.0.0:5558".to_string())
        .with_proto(Proto::Req)
        .with_handler(DummyCommand)
        .build()
        .connect()
        .into_inner();
    let push_clone = push.clone();

    let server = ConnectorNNG::builder()
        .with_endpoint("tcp://0.0.0.0:5557".to_string())
        .with_proto(Proto::Rep)
        .with_handler(DecoderCommand { push: push_clone })
        .build()
        .bind()
        .into_inner();

    thread::spawn(move || {
        Poller::new()
            .add(server)
            .add(push)
            .poll();
    }).join().unwrap();
}