use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_core::transport::connector_nng::ConnectorNNG;
use net_core::transport::polling::Poller;

use crate::command::decoder::DecoderCommand;
use crate::command::dummy::DummyCommand;
use crate::command::dispatcher::TranslatorDispatcher;
use crate::command::timescale_command::TimescaleCommand;
use crate::command::transmitter::Transmitter;

pub struct Translator {
    pub pool: ThreadPool,
}

impl Translator {
    pub fn new(pool: ThreadPool) -> Self {
        Self { pool }
    }
}

const PRODUCER: &'static str = "inproc://nng/dispatcher_producer";
const TRANSMITTER: &'static str = "inproc://nng/transmitter";

impl NetComponent for Translator {
    fn run(self) {
        log::info!("Run component");
        // inproc part
        // ========================
        let transmitter_sub = ConnectorNNG::pub_sub_builder()
            .with_endpoint(TRANSMITTER.to_owned())
            .with_handler(Transmitter)
            .build_subscriber()
            .bind()
            .into_inner();
        let transmitter_pub = ConnectorNNG::pub_sub_builder()
            .with_endpoint(TRANSMITTER.to_owned())
            .with_handler(DummyCommand)
            .build_publisher()
            .connect()
            .into_inner();
        let producer = ConnectorNNG::pub_sub_builder()
            .with_endpoint(PRODUCER.to_owned())
            .with_handler(DummyCommand)
            .build_publisher()
            .bind()
            .into_inner();
        let decoder = ConnectorNNG::pub_sub_builder()
            .with_endpoint(PRODUCER.to_string())
            .with_handler(DecoderCommand { transmitter: transmitter_pub })
            .with_topic("decode".as_bytes().to_owned())
            .build_subscriber()
            .connect()
            .into_inner();

        let db_service = ConnectorNNG::pub_sub_builder()
            .with_endpoint("tcp://0.0.0.0:5556".to_string())
            .with_handler(DummyCommand)
            .build_publisher()
            .bind()
            .into_inner();

        let timescale_command = ConnectorNNG::pub_sub_builder()
            .with_endpoint(PRODUCER.to_string())
            .with_handler(TimescaleCommand {producer: db_service})
            .with_topic("db".as_bytes().to_owned())
            .build_subscriber()
            .connect()
            .into_inner();
        //=========================================
        let dispatcher = TranslatorDispatcher { producer };
        let server = ConnectorNNG::pub_sub_builder()
            .with_endpoint("tcp://0.0.0.0:5557".to_string())
            .with_handler(dispatcher)
            .build_subscriber()
            .bind()
            .into_inner();

        self.pool.execute(move || {
            Poller::new()
                .add(server)
                .add(decoder)
                .add(timescale_command)
                .add(transmitter_sub)
                .poll();
        });
    }
}
