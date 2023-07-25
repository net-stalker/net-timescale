use std::ops::DerefMut;
use std::sync::Arc;

use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::pg::{Pg, PgConnection};
use net_core::transport::dummy_command::DummyCommand;
use net_core::transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
use net_core::transport::polling::zmq::ZmqPoller;
use net_core::transport::sockets::Context;
use net_core::transport::zmq::builders::publisher::ConnectorZmqPublisherBuilder;
use net_core::transport::zmq::builders::subscriber::ConnectorZmqSubscriberBuilder;
use net_core::transport::zmq::contexts::dealer::DealerContext;
use net_core::transport::zmq::contexts::publisher::PublisherContext;
use net_core::transport::zmq::contexts::subscriber::SubscriberContext;
use crate::command::{
    dispatcher::CommandDispatcher,
    executor::PoolWrapper,
    router::Router,
    network_packet_handler::NetworkPacketHandler,
    network_graph_handler::NetworkGraphHandler,
};
use crate::config::Config;
use crate::repository::continuous_aggregate;

pub struct Timescale {
    thread_pool: ThreadPool,
    connection_pool: Pool<ConnectionManager<PgConnection>>,
    config: Config,
}

impl Timescale {
    pub fn new(thread_pool: ThreadPool, config: Config) -> Self {
        let connection_pool = Timescale::configure_connection_pool(&config);
        Timescale::create_continuous_aggregate(connection_pool.get().unwrap().deref_mut());
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

    fn create_continuous_aggregate(con: &mut PgConnection) {
        match continuous_aggregate::create_address_pair_aggregate(con) {
            Ok(_) => {
                log::info!("successfully created address pair continuous aggregate");
            },
            Err(err) => {
                log::debug!("couldn't create an address pair continuous aggregate: {}", err);
            }
        }
        match continuous_aggregate::add_refresh_policy_for_address_pair_aggregate(con) {
            Ok(_) => {
                log::info!("successfully created a refresh policy for address pair continuous aggregate");
            },
            Err(err) => {
                log::debug!("couldn't create a refresh policy for address pair continuous aggregate: {}", err);
            }
        }
    }
}
// TODO: move this to the configuration in future
pub const TIMESCALE_CONSUMER: &'static str = "inproc://timescale/consumer";
pub const TIMESCALE_PRODUCER: &'static str = "inproc://timescale/producer";

impl NetComponent for Timescale {
    fn run(self) {
        log::info!("Run component");
        let dealer_context = DealerContext::default();
        let pub_context = PublisherContext::default();
        let sub_context = SubscriberContext::new(pub_context.get_context());
        let dealer_context_clone = dealer_context.clone();

        self.thread_pool.execute(move || {
            let consumer_db_service = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(self.config.timescale_endpoint.addr)
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let router_command = Router::new(consumer_db_service);
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Arc::new(router_command))
                .build()
                .bind()
                .into_inner();

            let consumer = ConnectorZmqPublisherBuilder::new(&pub_context)
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .bind()
                .into_inner();

            let dispatcher = CommandDispatcher::new(consumer);
            let producer_db_service = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(self.config.translator_connector.addr)
                .with_handler(Arc::new(dispatcher))
                .build()
                .bind()
                .into_inner();

            ZmqPoller::new()
                .add(router)
                .add(producer_db_service)
                .poll(-1);
        });
        let dealer_context_clone = dealer_context.clone();
        let sub_context_clone = sub_context.clone();
        let connection_pool = self.connection_pool.clone();

        self.thread_pool.execute(move || {
            // TODO: create zmq pub/sub connector. These connector must be able to use inproc proto
            let executor = PoolWrapper::new(connection_pool);
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let network_packet_handler = NetworkPacketHandler::new(executor.clone(),
                                                                   router.clone());
            let network_packet_connector = ConnectorZmqSubscriberBuilder::new(&sub_context_clone)
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(Arc::new(network_packet_handler))
                // TODO: add these topics to net-timescale-api
                .with_topic("network_packet".as_bytes().into())
                .build()
                .connect()
                .into_inner();

            ZmqPoller::new()
                .add(network_packet_connector)
                .poll(-1);
        });

        self.thread_pool.execute(move || {
            let executor = PoolWrapper::new(self.connection_pool);
            let router = ConnectorZmqDealerBuilder::new(&dealer_context)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let network_graph_handler = NetworkGraphHandler::new(executor.clone(),
                                                                 router.clone());
            let network_graph_connector = ConnectorZmqSubscriberBuilder::new(&sub_context)
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(Arc::new(network_graph_handler))
                .with_topic("date_cut".as_bytes().into())
                .build()
                .connect()
                .into_inner();
            ZmqPoller::new()
                .add(network_graph_connector)
                .poll(-1);
        })
    }
}