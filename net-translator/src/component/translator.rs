use std::env;
use net_core::transport::connector_nng_pub_sub::ConnectorNNGPubSub;
use net_core::transport::dummy_command::DummyCommand;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_core::transport::{
    connector_nng::{ConnectorNNG, Proto}
};
use net_core::transport::connector_zeromq::ConnectorZmq;
use net_core::transport::polling::nng::NngPoller;
use net_core::transport::polling::zmq::ZmqPoller;

use crate::command::decoder::DecoderCommand;
use crate::command::dispatcher::TranslatorDispatcher;
use crate::command::timescale_command::TimescaleCommand;
use crate::config::Config;

pub struct Translator {
    pool: ThreadPool,
    config: Config,
}

impl Translator {
    pub fn new(pool: ThreadPool, config: Config) -> Self {
        Self { pool, config }
    }
}

const DISPATCHER: &'static str = "inproc://dispatcher";
const DECODER: &'static str = "inproc://decoder";

impl NetComponent for Translator {
    fn run(self) {
        log::info!("Run component");
        self.pool.execute(move || {
            // build timescale command
            let timescale = ConnectorZmq::builder()
                .with_endpoint(self.config.translator_endpoint.addr)
                .with_handler(DummyCommand)
                .build()
                .bind()
                .into_inner();

            let db_command = ConnectorNNGPubSub::builder()
                .with_endpoint(DECODER.to_owned())
                .with_handler(TimescaleCommand { consumer: timescale })
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
                .with_handler(DecoderCommand { consumer: decoder_consumer })
                .with_proto(Proto::Pull)
                .build()
                .connect()
                .into_inner();

            NngPoller::new()
                .add(db_command)
                .add(decoder)
                .poll(-1);
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
            let dispatcher = ConnectorZmq::builder()
                .with_endpoint(self.config.translator_connector.addr)
                .with_handler(dispatcher_command)
                .build()
                .connect()
                .into_inner();

            ZmqPoller::new()
                .add(dispatcher)
                .poll(-1);
        });
    }
}
