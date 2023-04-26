use postgres::NoTls;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;
use crate::command::{dispatcher::CommandDispatcher, executor::Executor, result_sender::ResultSender};
use crate::db_access::add_traffic::add_captured_packets::AddCapturedPackets;
use crate::db_access::query_factory::QueryFactory;
use crate::db_access::select_by_time::select_by_time::SelectInterval;

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

pub const DISPATCHER_PUBSLIHER: &'static str = "inproc://nng/dispatcher";
pub const RESULT_PULLER: &'static str = "inproc://nng/result_sender";

impl NetComponent for Timescale {
    fn run(self) {
        log::info!("Run component");
        self.thread_pool.execute(move || {
            let dispatcher = CommandDispatcher::builder()
                .with_endpoint(DISPATCHER_PUBSLIHER.to_owned())
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
            let handler_result_sender = ResultSender {
                connector: db_service.clone()
            };
            let result_sender = ConnectorNNG::builder()
                .with_endpoint(RESULT_PULLER.to_owned())
                .with_proto(Proto::Pull)
                .with_handler(handler_result_sender)
                .build()
                .bind()
                .into_inner();

            Poller::new()
                .add(db_service)
                .add(result_sender)
                .poll();
        });

        self.thread_pool.execute( move|| {
            let executor = Executor::new(self.connection_pool.clone());
            let add_packets_handler = AddCapturedPackets::create_query_handler(executor.clone(), RESULT_PULLER);
            let service_add_packets = ConnectorNNG::pub_sub_builder()
                .with_endpoint(DISPATCHER_PUBSLIHER.to_owned())
                .with_handler(add_packets_handler)
                .with_topic("add_packet".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();
            
            let select_by_time_interval_handler = SelectInterval::create_query_handler(executor.clone(), RESULT_PULLER);
            let service_select_by_time_interval = ConnectorNNG::pub_sub_builder()
                .with_endpoint(DISPATCHER_PUBSLIHER.to_owned())
                .with_handler(select_by_time_interval_handler)
                .with_topic("select_time".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();

            Poller::new()
                .add(service_add_packets)
                .add(service_select_by_time_interval)
                .poll();
        });
    }
}