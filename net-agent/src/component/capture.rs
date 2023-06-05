use log::info;
use net_core::transport::dummy_command::DummyCommand;
use threadpool::ThreadPool;

use net_core::capture;
use net_core::layer::NetComponent;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};

use crate::codec::Codec;

pub struct Capture {
    pool: ThreadPool,
}

impl Capture {
    pub fn new(pool: ThreadPool) -> Self {
        Capture {
            pool,
        }
    }
}

impl NetComponent for Capture {
    fn run(self) {
        info!("Run component");
        let capture = pcap::Capture::from_device("en0")
            .unwrap()
            // .promisc(true)
            // .snaplen(65535)
            .buffer_size(1000)
            .open()
            .unwrap();
        self.pool.execute(move || {
            let client = ConnectorNNG::builder()
            .with_endpoint("tcp://0.0.0.0:5555".to_string())
            .with_proto(Proto::Push)
            .with_handler(DummyCommand)
            .build()
            .connect()
            .into_inner();

            let codec = Codec::new(client);
            capture::polling::Poller::new(capture)
                .with_packet_cnt(1)
                .with_codec(codec)
                .poll();
        });
    }
}