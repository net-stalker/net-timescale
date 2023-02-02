use std::sync::{Arc, RwLock};
use std::thread;
use net_core::capture::decoder_binary::BinaryDecoder;
use net_core::translator::Decoder;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;
use net_translator::command::hub::HubCommand;

fn main() {
    let client = ConnectorNNG::builder()
        .with_endpoint("tcp://0.0.0.0:5557".to_string())
        .with_proto(Proto::Rep)
        .with_handler(HubCommand)
        .build()
        .bind()
        .into_inner();

    thread::spawn(move || {
        Poller::new()
            .add(client)
            .poll();
    }).join().unwrap();
}