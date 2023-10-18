use std::collections::HashSet;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::time::Duration;
use async_std::task::block_on;

use net_proto_api::typed_api::Typed;
use net_timescale_api::api::dashboard_request::DashboardRequestDTO;

use net_timescale_api::api::network_graph_request::NetworkGraphRequestDTO;

use net_transport::dummy_command::DummyCommand;
use threadpool::ThreadPool;
use sqlx::{
    Postgres,
    postgres::PgPoolOptions,
    Pool,
};
use net_transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
use net_transport::polling::zmq::ZmqPoller;
use net_transport::sockets::Context;
use net_transport::zmq::builders::publisher::ConnectorZmqPublisherBuilder;
use net_transport::zmq::builders::subscriber::ConnectorZmqSubscriberBuilder;
use net_transport::zmq::contexts::dealer::DealerContext;
use net_transport::zmq::contexts::publisher::PublisherContext;
use net_transport::zmq::contexts::subscriber::SubscriberContext;
use crate::command::{
    dispatcher::CommandDispatcher,
    executor::PoolWrapper,
    router::Router,
    network_packet_handler::NetworkPacketHandler,
};

use crate::command::dashboard::handler::DashboardHandler;

use crate::command::listen_handler::ListenHandler;
use crate::config::Config;
use crate::repository::continuous_aggregate::bandwidth_per_endpoint::BandwidthPerEndpointAggregate;
use crate::repository::continuous_aggregate::network_graph::NetworkGraphAggregate;

pub const TIMESCALE_CONSUMER: &str = "inproc://timescale/consumer";
pub const TIMESCALE_PRODUCER: &str = "inproc://timescale/producer";
pub const IS_REALTIME: &str = "inproc://timescale/is-realtime";
pub const DASHBOARD_HANDLER: &str = "inproc://timescale/dashboard-handler";

pub struct Timescale {
    thread_pool: ThreadPool,
    connection_pool: Pool<Postgres>,
    config: Config,
}

impl Timescale {
    pub async fn new(thread_pool: ThreadPool, config: Config) -> Self {
        let connection_pool = Timescale::configure_connection_pool(&config).await;
        Timescale::create_continuous_aggregates(&connection_pool).await;
        Self {
            thread_pool,
            connection_pool,
            config
        }
    }
    async fn configure_connection_pool(config: &Config) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(config.max_connection_size.size.parse().expect("not a number"))
            .connect(config.connection_url.url.as_str())
            .await
            .unwrap()
    }

    async fn create_continuous_aggregates(con: &Pool<Postgres>) {
        // TODO: think of refactoring this method using HashMap probably
        match NetworkGraphAggregate::create(con).await {
            Ok(_) => {
                log::info!("successfully created address pair continuous aggregate");
            },
            Err(err) => {
                log::debug!("couldn't create {}: {}", NetworkGraphAggregate::get_name(), err);
            }
        }
        match NetworkGraphAggregate::add_refresh_policy(con, None, None, "1 minute").await {
            Ok(_) => {
                log::info!("successfully created {} refresh policy", NetworkGraphAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {} refresh policy: {}", NetworkGraphAggregate::get_name(), err);
            }
        }
        match BandwidthPerEndpointAggregate::create(con).await {
            Ok(_) => {
                log::info!("successfully created {}", BandwidthPerEndpointAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {}: {}", BandwidthPerEndpointAggregate::get_name(), err);
            }
        }
        match BandwidthPerEndpointAggregate::add_refresh_policy(con, None, None, "1 minute").await {
            Ok(_) => {
                log::info!("successfully created {} refresh policy", BandwidthPerEndpointAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {} refresh policy: {}", BandwidthPerEndpointAggregate::get_name(), err);
            }
        }
    }

    pub async fn run(self) {
        log::info!("Run component"); 
        let dealer_context = DealerContext::default();
        let pub_context = PublisherContext::default();
        let sub_context = SubscriberContext::new(pub_context.get_context());
        let dealer_context_clone = dealer_context.clone();

        self.thread_pool.execute(move || {
            let consumer_db_service = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(self.config.timescale_endpoint.addr)
                .with_handler(Rc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let router_command = Router::new(consumer_db_service);
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Rc::new(router_command))
                .build()
                .bind()
                .into_inner();

            let consumer = ConnectorZmqPublisherBuilder::new(&pub_context)
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(Rc::new(DummyCommand))
                .build()
                .bind()
                .into_inner();

            let dispatcher = CommandDispatcher::new(consumer);
            let producer_db_service = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(self.config.translator_connector.addr)
                .with_handler(Rc::new(dispatcher))
                .build()
                .bind()
                .into_inner();

            ZmqPoller::new()
                .add(router)
                .add(producer_db_service)
                .poll(-1);
        });
        let dealer_context_clone = dealer_context.clone();
        let connection_pool_clone = self.connection_pool.clone();
        // TODO: create a wrapper for tenants
        let tenants = Arc::new(Mutex::new(
            Arc::new(async_std::sync::RwLock::new(HashSet::default()))
        ));
        let tenants_clone = tenants.clone();
        self.thread_pool.execute(move || {
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Rc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();
            let pool = PoolWrapper::new(connection_pool_clone);
            let mut listen_handler = ListenHandler::builder()
                .with_connection_pool(pool)
                .with_router(router)
                .build();
            if let Ok(mut guard) = tenants_clone.lock() {
                let temp = guard.deref_mut();
                *temp = listen_handler.get_tenants();
            }
            block_on(listen_handler.start("insert_channel", -1));
        });

        std::thread::sleep(Duration::from_secs(1));
        let sub_context_clone = sub_context.clone();
        let connection_pool_clone = self.connection_pool.clone();

        self.thread_pool.execute(move || {
            let executor = PoolWrapper::new(connection_pool_clone).into_inner();
            let network_packet_handler = NetworkPacketHandler::new(executor.clone(),
            "dummy".to_string());
            let network_packet_connector = ConnectorZmqSubscriberBuilder::new(&sub_context_clone)
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(Rc::new(network_packet_handler))
                // TODO: add these topics to net-timescale-api
                .with_topic("network_packet".as_bytes().into())
                .build()
                .connect()
                .into_inner();

            ZmqPoller::new()
                .add(network_packet_connector)
                .poll(-1);
        });
        let connection_pool = self.connection_pool.clone();
        let dealer_context_clone = dealer_context.clone();

        self.thread_pool.execute(move || {
            let pool = PoolWrapper::new(connection_pool).into_inner();
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Rc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let dashboard_handler = DashboardHandler::builder()
                .with_consumer(router)
                .with_pool(pool)
                .add_chart_constructor(
                    (NetworkGraphRequestDTO::get_data_type(),
                     crate::persistence::network_graph::get_network_graph_for_dashboard
                    )
                )
                .build();
            let dashboard_connector = ConnectorZmqSubscriberBuilder::new(&sub_context)
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_topic(DashboardRequestDTO::get_data_type().as_bytes().to_owned())
                .with_handler(Rc::new(dashboard_handler))
                .build()
                .connect()
                .into_inner();
            ZmqPoller::new()
                .add(dashboard_connector)
                .poll(-1);
        });
    }
}