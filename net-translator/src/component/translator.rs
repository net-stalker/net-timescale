use net_core::transport::connector_nng_pub_sub::ConnectorNNGPubSub;
use net_core::transport::dummy_command::DummyCommand;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_core::transport::{
    connector_nng::{ConnectorNNG, Proto}
};
use net_core::transport::polling::nng::NngPoller;

use crate::command::decoder::DecoderCommand;
use crate::command::dispatcher::TranslatorDispatcher;
use crate::command::timescale_command::TimescaleCommand;

pub struct Translator {
    pub pool: ThreadPool,
}

impl Translator {
    pub fn new(pool: ThreadPool) -> Self {
        Self { pool }
    }
}

const DISPATCHER: &'static str = "inproc://dispatcher";
const DECODER: &'static str = "inproc://decoder";

impl NetComponent for Translator {
    fn run(self) {
        log::info!("Run component");
        self.pool.execute(move || {
            // build timescale command
            let timescale = ConnectorNNG::builder()
                .with_proto(Proto::Push)
                .with_endpoint("tcp://0.0.0.0:5556".to_string())
                .with_handler(DummyCommand)
                .build()
                .bind()
                .into_inner();

            let db_command = ConnectorNNGPubSub::builder()
                .with_endpoint(DECODER.to_owned())
                .with_handler(TimescaleCommand {consumer: timescale})
                .build_subscriber()
                .connect()
                .into_inner();

            let decoder_consumer = ConnectorNNGPubSub::builder()
                .with_endpoint(DECODER.to_owned())
                .with_handler(DummyCommand)
                .build_publisher()
                .bind()
                .into_inner();

            let decoder = ConnectorNNG::builder()
                .with_endpoint(DISPATCHER.to_owned())
                .with_handler(DecoderCommand {consumer: decoder_consumer})
                .with_proto(Proto::Pull)
                .build()
                .connect()
                .into_inner();

            NngPoller::new()
                .add(db_command)
                .add(decoder)
                .poll();
        });

        self.pool.execute(move || {

            let consumer = ConnectorNNG::builder()
                .with_endpoint(DISPATCHER.to_owned())
                .with_handler(DummyCommand)
                .with_proto(Proto::Push)
                .build()
                .bind()
                .into_inner();     
            
            let dispatcher_command = TranslatorDispatcher { consumer };
            let dispatcher = ConnectorNNGPubSub::builder()
                .with_endpoint("tcp://0.0.0.0:5557".to_string())
                .with_handler(dispatcher_command)
                .build_subscriber()
                .connect()
                .into_inner();

            NngPoller::new()
                .add(dispatcher)
                .poll();
        });
    }
}
