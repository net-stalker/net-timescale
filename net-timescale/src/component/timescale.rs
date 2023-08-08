use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use async_std::task::block_on;
use chrono::{TimeZone, Utc};

use threadpool::ThreadPool;
use sqlx::{
    Postgres,
    postgres::PgPoolOptions,
    Pool,
};
use net_transport::{
    dummy_command::DummyCommand,
};
use net_transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
use net_transport::polling::zmq::ZmqPoller;
use net_transport::sockets::Context;
use net_transport::zmq::builders::publisher::ConnectorZmqPublisherBuilder;
use net_transport::zmq::builders::subscriber::ConnectorZmqSubscriberBuilder;
use net_transport::zmq::contexts::dealer::DealerContext;
use net_proto_api::{
    decoder_api::Decoder,
    encoder_api::Encoder,
    envelope::envelope::Envelope,
};
use net_transport::zmq::contexts::publisher::PublisherContext;
use net_transport::zmq::contexts::subscriber::SubscriberContext;
use crate::command::{
    dispatcher::CommandDispatcher,
    executor::PoolWrapper,
    router::Router,
    network_packet_handler::NetworkPacketHandler,
    network_graph_handler::NetworkGraphHandler,
};
use crate::command::realtime_handler::IsRealtimeHandler;
use crate::command::listen_command::listen_handler::ListenHandler;
use crate::command::listen_command::realtime_notifications_handler;
use crate::command::listen_command::realtime_notifications_handler::RealtimeNotificationHandler;
use crate::config::Config;
use crate::repository::continuous_aggregate;

pub const TIMESCALE_CONSUMER: &str = "inproc://timescale/consumer";
pub const TIMESCALE_PRODUCER: &str = "inproc://timescale/producer";
pub const IS_REALTIME: &str = "inproc://timescale/is-realtime";
pub const LISTEN_HANDLER: &str = "inproc://timescale/listen-handler";

pub struct Timescale {
    thread_pool: ThreadPool,
    connection_pool: Pool<Postgres>,
    config: Config,
}

impl Timescale {
    pub async fn new(thread_pool: ThreadPool, config: Config) -> Self {
        let connection_pool = Timescale::configure_connection_pool(&config).await;
        Timescale::create_continuous_aggregate(&connection_pool).await;
        Self {
            thread_pool,
            connection_pool,
            config
        }
    }
    async fn configure_connection_pool(config: &Config) -> Pool<Postgres> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connection_size.size.parse().expect("not a number"))
            .connect(config.connection_url.url.as_str())
            .await
            .unwrap();
        pool
    }

    async fn create_continuous_aggregate<'e>(con: &'e Pool<Postgres>) {
        match continuous_aggregate::create_address_pair_aggregate(con).await {
            Ok(_) => {
                log::info!("successfully created address pair continuous aggregate");
            },
            Err(err) => {
                log::debug!("couldn't create an address pair continuous aggregate: {}", err);
            }
        }
        match continuous_aggregate::add_refresh_policy_for_address_pair_aggregate(con).await {
            Ok(_) => {
                log::info!("successfully created a refresh policy for address pair continuous aggregate");
            },
            Err(err) => {
                log::debug!("couldn't create a refresh policy for address pair continuous aggregate: {}", err);
            }
        }
    }

    pub async fn run(self) {
        log::info!("Run component");
        let dealer_context = DealerContext::default();
        let pub_context = PublisherContext::default();
        let pub_context_clone = pub_context.clone();

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

            let consumer = ConnectorZmqPublisherBuilder::new(&pub_context_clone)
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
        let connection_pool_clone = self.connection_pool.clone();
        let dealer_context_clone = dealer_context.clone();
        let (sender, receiver) = async_channel::unbounded();
        let sender_clone = sender.clone();
        let receiver_clone = receiver.clone();
        self.thread_pool.execute(move || {
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(LISTEN_HANDLER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .bind()
                .into_inner();
            let pool = PoolWrapper::new(connection_pool_clone);
            let mut listen_handler = ListenHandler::builder()
                .with_connection_pool(pool)
                .with_router(router)
                .with_receiver(receiver_clone)
                .with_sender(sender_clone)
                .build();
            block_on(listen_handler.poll(-1));
        });
        let pool = self.connection_pool.clone();
        let dealer_context_clone = dealer_context.clone();
        let connections = Arc::new(RwLock::new(HashSet::default()));
        let connections_clone = connections.clone();
        self.thread_pool.execute(move || {
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();
            let realtime_notification_handler = RealtimeNotificationHandler::new(
                PoolWrapper::new(pool).into_inner(),
                router,
                connections_clone
            );
            let realtime_notification_connector =
                ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(LISTEN_HANDLER.to_owned())
                .with_handler(Arc::new(realtime_notification_handler))
                .build()
                .connect()
                .into_inner();
            ZmqPoller::new()
                .add(realtime_notification_connector)
                .poll(-1);
        });
        std::thread::sleep(Duration::from_secs(1));
        let dealer_context_clone = dealer_context.clone();
        let sub_context_clone = sub_context.clone();
        let connection_pool_clone = self.connection_pool.clone();

        self.thread_pool.execute(move || {
            let executor = PoolWrapper::new(connection_pool_clone).into_inner();
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let network_packet_handler = NetworkPacketHandler::new(executor.clone(),
            "dummy".to_string());
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
        let connection_pool = self.connection_pool.clone();
        let dealer_context_clone = dealer_context.clone();
        self.thread_pool.execute(move || {
            let executor = PoolWrapper::new(connection_pool).into_inner();
            let router = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let is_realtime_consumer = ConnectorZmqDealerBuilder::new(&dealer_context_clone)
                .with_endpoint(IS_REALTIME.to_owned())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();
            let network_graph_handler = Arc::new(NetworkGraphHandler::new(
                executor.clone(),
                router.clone(),
                is_realtime_consumer.clone()
            ));
            // TODO: check if we can set multiple topics to a single socket
            let network_graph_connector = ConnectorZmqSubscriberBuilder::new(&sub_context)
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(network_graph_handler.clone())
                .with_topic("NG_request".as_bytes().into())
                .build()
                .connect()
                .into_inner();
            let is_realtime_handler = IsRealtimeHandler::new(
                connections,
                sender
            );
            let is_realtime_connector = ConnectorZmqDealerBuilder::new(&dealer_context)
                .with_endpoint(IS_REALTIME.to_owned())
                .with_handler(Arc::new(is_realtime_handler))
                .build()
                .bind()
                .into_inner();
            ZmqPoller::new()
                .add(network_graph_connector)
                .add(is_realtime_connector)
                .poll(-1);
        });
    }
}
