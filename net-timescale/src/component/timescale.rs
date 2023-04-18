use postgres::NoTls;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;
use crate::command::dispatcher::CommandDispatcher;
use crate::command::executor::Executor;
use crate::db_access::add_traffic::add_captured_packets::AddCapturedPackets;

pub struct Timescale {
    pub thread_pool: ThreadPool,
    // for now we specify `NoTls` just for simplicity
    pub connection_pool: Pool<PostgresConnectionManager<NoTls>>
}

impl Timescale {
    pub fn new(thread_pool: ThreadPool, connection_pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self {
            thread_pool,
            connection_pool
        }
    }
}
const PUBLISHER: &'static str = "inproc://nng/dispatcher";

impl NetComponent for Timescale {
    fn run(self) {
        log::info!("Run component");
        self.thread_pool.execute(move || {
            let dispatcher = CommandDispatcher::builder()
                .with_endpoint(PUBLISHER.to_owned())
                .with_proto(Proto::Pub)
                .build()
                .bind();

            let db_service = ConnectorNNG::builder()
                .with_endpoint("tcp://0.0.0.0:5556".to_string())
                .with_proto(Proto::Rep)
                .with_handler(dispatcher)
                .build()
                .bind()
                .into_inner();

            Poller::new()
                .add(db_service)
                .poll();
        });

        self.thread_pool.execute( move|| {
            let executor = Executor::new(self.connection_pool.clone());
            let add_packets_handler = AddCapturedPackets { executor: executor.clone() };
            let packets_service = ConnectorNNG::builder()
                .with_endpoint(PUBLISHER.to_owned())
                .with_proto(Proto::Sub)
                .with_handler(add_packets_handler)
                .with_topic("add_packet".as_bytes().into())
                .build()
                .connect()
                .into_inner();
            Poller::new()
                .add(packets_service)
                .poll();
        });
    }
}