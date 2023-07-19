use std::env;
use std::sync::Arc;
use net_core::transport::dummy_command::DummyCommand;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_core::transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
use net_core::transport::polling::zmq::ZmqPoller;
use net_core::transport::sockets::Context;
use net_core::transport::zmq::builders::publisher::ConnectorZmqPublisherBuilder;
use net_core::transport::zmq::builders::subscriber::ConnectorZmqSubscriberBuilder;
use net_core::transport::zmq::contexts::dealer::DealerContext;
use net_core::transport::zmq::contexts::publisher::PublisherContext;
use net_core::transport::zmq::contexts::subscriber::SubscriberContext;

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

const DISPATCHER: &'static str = "inproc://translator/dispatcher";
const DECODER: &'static str = "inproc://translator/decoder";

impl NetComponent for Translator {
    fn run(self) {
        log::info!("Run component");
        let dealer_context = DealerContext::default();
        let dealer_context_clone = dealer_context.clone();
        let sub_context = SubscriberContext::default();
        let pub_context = PublisherContext::new(sub_context.get_context());
        self.pool.execute(move || {
            // build timescale command
            let timescale = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(self.config.translator_endpoint.addr)
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let db_command = ConnectorZmqSubscriberBuilder::new(&sub_context)
                .with_endpoint(DECODER.to_owned())
                .with_handler(Arc::new(TimescaleCommand { consumer: timescale }))
                .build()
                .connect()
                .into_inner();

            let decoder_consumer = ConnectorZmqPublisherBuilder::new(&pub_context)
                .with_endpoint(DECODER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .bind()
                .into_inner();

            let decoder = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(DISPATCHER.to_owned())
                .with_handler(Arc::new(DecoderCommand { consumer: decoder_consumer }))
                .build()
                .connect()
                .into_inner();

            ZmqPoller::new()
                .add(db_command)
                .add(decoder)
                .poll(-1);
        });
        let dealer_context_clone = dealer_context.clone();
        self.pool.execute(move || {
            let consumer = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(DISPATCHER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .bind()
                .into_inner();

            let dispatcher_command = TranslatorDispatcher { consumer };
            let dispatcher = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(self.config.translator_connector.addr)
                .with_handler(Arc::new(dispatcher_command))
                .build()
                .connect()
                .into_inner();

            ZmqPoller::new()
                .add(dispatcher)
                .poll(-1);
        });
    }
}
