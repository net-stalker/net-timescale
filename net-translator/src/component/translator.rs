use net_core::topic::{DECODER_TOPIC, DB_TOPIC};
use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_core::transport::connector_nng::{ConnectorNNG, Proto};
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

const CONSUMER: &'static str = "inproc://nng/dispatcher_consumer";
const TRANSMITTER: &'static str = "inproc://nng/transmitter";

impl NetComponent for Translator {
    fn run(self) {
        log::info!("Run component");
        // consumer - a connection to send data back to hub
        let consumer = ConnectorNNG::pub_sub_builder()
            .with_endpoint("tcp://0.0.0.0:5555".to_string())
            .with_handler(DummyCommand)
            .build_publisher()
            .connect()
            .into_inner();
        //  transmitter_sub - connector which sends data received from sub via network
        let transmitter_sub = ConnectorNNG::pub_sub_builder()
            .with_endpoint(TRANSMITTER.to_owned())
            .with_handler(Transmitter {consumer})
            .build_subscriber()
            .bind()
            .into_inner();
        // transmitter_pub - connector publish data to transmitter sub 
        let transmitter_pub = ConnectorNNG::pub_sub_builder()
            .with_endpoint(TRANSMITTER.to_owned())
            .with_handler(DummyCommand)
            .build_publisher()
            .connect()
            .into_inner();
             // consumer - local publisher which sends data from dispatcher to local services
        let consumer = ConnectorNNG::pub_sub_builder()
             .with_endpoint(CONSUMER.to_owned())
             .with_handler(DummyCommand)
             .build_publisher()
             .bind()
             .into_inner();
        // decoder - local service which decodes data
        let decoder = ConnectorNNG::pub_sub_builder()
            .with_endpoint(CONSUMER.to_string())
            .with_handler(DecoderCommand { transmitter: transmitter_pub })
            .with_topic(DECODER_TOPIC.as_bytes().to_owned())
            .build_subscriber()
            .connect()
            .into_inner();
        // db_service - a server which allows `net-timescale` modules to connect to itself
        let db_service = ConnectorNNG::pub_sub_builder()
            .with_endpoint("tcp://0.0.0.0:5556".to_string())
            .with_handler(DummyCommand)
            .build_publisher()
            .bind()
            .into_inner();
        // timescale_command - a local service which sends data to a connected `net-timescale` module 
        let timescale_command = ConnectorNNG::pub_sub_builder()
            .with_endpoint(CONSUMER.to_string())
            .with_handler(TimescaleCommand {consumer: db_service})
            .with_topic(DB_TOPIC.as_bytes().to_owned())
            .build_subscriber()
            .connect()
            .into_inner();
        // dispatcher is a dispatcher which holds a producer :)
        let dispatcher = TranslatorDispatcher { consumer };
        // server - a server which basically can receive connections from all the modules except `net-timescale`. At least for now
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
