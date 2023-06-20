use std::ops::DerefMut;
use std::sync::Arc;

use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::pg::{Pg, PgConnection};
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
    executor::PoolWrapper,
    router::Router,
    network_packet_handler::NetworkPacketHandler,
    network_graph_handler::NetworkGraphHandler,
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
            let consumer_db_service = ConnectorZmq::builder()
                .with_endpoint(self.config.timescale_endpoint.addr)
                .with_handler(DummyCommand)
                .build()
                .connect()
                .into_inner();

            let router_command = Router::new(consumer_db_service);
            let router = ConnectorZmq::builder()
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(router_command)
                .build()
                .bind()
                .into_inner();

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
                .bind()
                .into_inner();

            ZmqPoller::new()
                .add(router)
                .add(producer_db_service)
                .poll(-1);
        });
        self.thread_pool.execute(move || {
            // TODO: create zmq pub/sub connector. These connector must be able to use inproc proto
            let executor = PoolWrapper::new(self.connection_pool);
            let router = ConnectorZmq::builder()
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(DummyCommand)
                .build()
                .connect()
                .into_inner();

            let network_packet_handler = NetworkPacketHandler::new(executor.clone(),
                                                                   router.clone());
            let network_packet_connector = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(network_packet_handler)
                // TODO: add these topics to net-timescale-api
                .with_topic("network_packet".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();
            let network_graph_handler = NetworkGraphHandler::new(executor.clone(),
                                                                 router.clone());
            let network_graph_connector = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(network_graph_handler)
                .with_topic("network_graph".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();

            NngPoller::new()
                .add(network_packet_connector)
                .add(network_graph_connector)
                .poll(-1);
        });
    }
}