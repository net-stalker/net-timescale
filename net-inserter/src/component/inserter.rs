use std::rc::Rc;
use threadpool::ThreadPool;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use net_transport::polling::zmq::ZmqPoller;
use net_transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
use net_transport::zmq::contexts::dealer::DealerContext;
use crate::command::executor::PoolWrapper;
use crate::command::insert_handler::InsertHandler;
use crate::config::Config;

pub struct Inserter {
    thread_pool: ThreadPool,
    connection_pool: Pool<Postgres>,
    #[allow(dead_code)]
    config: Config,
}

impl Inserter {
    pub async fn new(thread_pool: ThreadPool, config: Config) -> Self {
        let connection_pool = Inserter::configure_connection_pool(&config).await;
        Self {
            thread_pool,
            connection_pool,
            config
        }
    }

    // TODO: move to builder
    async fn configure_connection_pool(config: &Config) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(config.max_connection_size.size.parse().expect("not a number"))
            .connect(config.connection_url.url.as_str())
            .await
            .unwrap()
    }

    pub async fn run(self) {
        log::info!("Run component"); 
        let dealer_context = DealerContext::default();

        self.thread_pool.execute(move || {
            let network_packet_handler = InsertHandler::new(
                PoolWrapper::new(self.connection_pool).into_inner()
            );

            let network_packet_connector = ConnectorZmqDealerBuilder::new(&dealer_context)
                .with_endpoint(self.config.hub_connector.addr)
                .with_handler(Rc::new(network_packet_handler))
                .build()
                .connect()
                .into_inner();

            ZmqPoller::new()
                .add(network_packet_connector)
                .poll(-1);
        });
    }
}