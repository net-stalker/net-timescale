use std::sync::Arc;

use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::pg::PgConnection;
use net_core::transport::{
    connector_nng::{ConnectorNNG, Proto},
    connector_nng_pub_sub::ConnectorNNGPubSub,
    dummy_command::DummyCommand,
    polling::nng::NngPoller
};
use net_core::transport::connector_zeromq::ConnectorZmq;
use net_core::transport::polling::zmq::ZmqPoller;
use crate::command::{
    dispatcher::CommandDispatcher,
    executor::PoolWrapper, transmitter::Transmitter
};
use crate::persistence::{
    network_packet::handler::NetworkPacketHandler,
};
use crate::config::Config;

pub struct Timescale {
    thread_pool: ThreadPool,
    connection_pool: Pool<ConnectionManager<PgConnection>>,
    config: Config,
}

impl Timescale {
    pub fn new(thread_pool: ThreadPool, config: Config) -> Self {
        let connection_pool = Timescale::configure_connection_pool(&config);
        Self {
            thread_pool,
            connection_pool,
            config
        }
    }
    fn configure_connection_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
        let manager = ConnectionManager::<PgConnection>::new(config.connection_url.url.clone());
        Pool::builder()
            .max_size(config.max_connection_size.size.parse().expect("not a number"))
            .test_on_check_out(true)
            .build(manager)
            .expect("could not build connection pool")
    }
}
// TODO: move this to the configuration in future
pub const TIMESCALE_CONSUMER: &'static str = "inproc://timescale/consumer";
pub const TIMESCALE_PRODUCER: &'static str = "inproc://timescale/producer";

impl NetComponent for Timescale {
    fn run(self) {
        log::info!("Run component");
        self.thread_pool.execute(move || {
            let consumer_db_service = ConnectorNNG::builder()
                .with_endpoint(self.config.timescale_endpoint.addr)
                .with_handler(DummyCommand)
                .with_proto(Proto::Push)
                .build()
                .connect()
                .into_inner(); 
            let transmitter_command = Transmitter::new(consumer_db_service);
            let transmitter = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(transmitter_command)
                .build_subscriber()
                .bind()
                .into_inner();
            NngPoller::new()
                .add(transmitter)
                .poll(-1);
        });
        self.thread_pool.execute(move || {
            let consumer = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(DummyCommand)
                .build_publisher()
                .bind()
                .into_inner();

            let dispatcher = CommandDispatcher::new(consumer);
            let producer_db_service = ConnectorZmq::builder()
                .with_endpoint(self.config.translator_connector.addr)
                .with_handler(dispatcher)
                .build()
                .connect()
                .into_inner();
            ZmqPoller::new()
                .add(producer_db_service)
                .poll(-1);
        });
        self.thread_pool.execute(move || {
            let executor = PoolWrapper::new(self.connection_pool.clone());
            let result_puller = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(DummyCommand)
                .build_publisher()
                .connect()
                .into_inner();

            let add_packets_handler = NetworkPacketHandler::new(executor.clone(),
                    result_puller.clone());
            let service_add_packets = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(add_packets_handler)
                .with_topic("network_packet".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();

            NngPoller::new()
                .add(service_add_packets)
                .poll(-1);
        });
    }
}