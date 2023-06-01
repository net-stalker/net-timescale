use postgres::NoTls;
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use net_core::transport::{
    connector_nng::{ConnectorNNG, Proto},
    connector_nng_pub_sub::ConnectorNNGPubSub,
    polling::Poller,
    dummy_command::DummyCommand,
};
use crate::command::{
    dispatcher::CommandDispatcher,
    executor::Executor, transmitter::Transmitter
};
use crate::db_access::{
    add_traffic::add_captured_packets::AddCapturedPackets,
    query_factory::QueryFactory,
    select_by_time::select_by_time::SelectInterval
};

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
// TODO: move this to the configuration in future
pub const TIMESCALE_CONSUMER: &'static str = "inproc://timescale/consumer";
pub const TIMESCALE_PRODUCER: &'static str = "inproc://timescale/producer";

impl NetComponent for Timescale {
    fn run(self) {
        log::info!("Run component");
        self.thread_pool.execute(move || {
            let consumer = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(DummyCommand)
                .build_publisher()
                .bind()
                .into_inner();

            let dispatcher = CommandDispatcher::new(consumer);
            let producer_db_service = ConnectorNNG::builder()
                .with_endpoint("tcp://0.0.0.0:5556".to_string())
                .with_handler(dispatcher)
                .with_proto(Proto::Pull)
                .build()
                .connect()
                .into_inner();

            let consumer_db_service = ConnectorNNG::builder()
                .with_endpoint("tcp://0.0.0.0:5558".to_string())
                .with_handler(DummyCommand)
                .with_proto(Proto::Push)
                .build()
                .connect()
                .into_inner(); 
            let trasmitter_command = Transmitter::new(consumer_db_service);
            let transmitter = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(trasmitter_command)
                .build_subscriber()
                .bind()
                .into_inner();
            Poller::new()
                .add(transmitter)
                .add(producer_db_service)
                .poll();
        });
        self.thread_pool.execute(move || {
            let executor = Executor::new(self.connection_pool.clone());
            let result_puller = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_PRODUCER.to_owned())
                .with_handler(DummyCommand)
                .build_publisher()
                .connect()
                .into_inner();

            let add_packets_handler = AddCapturedPackets::create_query_handler(executor.clone(),
                    result_puller.clone());
            let service_add_packets = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
                .with_handler(add_packets_handler)
                .with_topic("network_packet".as_bytes().into())
                .build_subscriber()
                .connect()
                .into_inner();
            
            let select_by_time_interval_handler = SelectInterval::create_query_handler(executor.clone(), result_puller.clone());
            let service_select_by_time_interval = ConnectorNNGPubSub::builder()
                .with_endpoint(TIMESCALE_CONSUMER.to_owned())
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