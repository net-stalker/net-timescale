use std::thread;
use std::thread::JoinHandle;

use log::info;
use shaku::{module, Component};

use net_core::starter::starter::Starter;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;

use crate::command::decoder::DecoderCommand;
use crate::command::dummy::DummyCommand;

module! {
    pub TranslatorModule {
        components = [Translator],
        providers = []
    }
}

#[derive(Component)]
#[shaku(interface = Starter)]
pub struct Translator;

impl Starter for Translator {
    fn start(&self) -> JoinHandle<()> {
        info!("Start module");

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
        })
    }
}
