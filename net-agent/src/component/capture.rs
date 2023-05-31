use log::info;
use net_core::transport::dummy_command::DummyCommand;
use pcap::Active;
use threadpool::ThreadPool;

use net_core::capture;
use net_core::layer::NetComponent;
use net_core::transport::connector_nng::ConnectorNNG;

use crate::codec::Codec;

pub struct Capture {
    capture: pcap::Capture<Active>,
    codec: Codec,
    pool: ThreadPool,
}

impl Capture {
    pub fn new(pool: ThreadPool) -> Self {
        let capture = pcap::Capture::from_device("en0")
            .unwrap()
            // .promisc(true)
            // .snaplen(65535)
            .buffer_size(1000)
            .open()
            .unwrap();
        
        let client = ConnectorNNG::pub_sub_builder()
            .with_endpoint("tcp://0.0.0.0:5555".to_string())
            .with_handler(DummyCommand)
            .build_publisher()
            .connect()
            .into_inner();

        let codec = Codec::new(client);

        Capture {
            capture,
            codec,
            pool,
        }
    }
}

impl NetComponent for Capture {
    fn run(self) {
        info!("Run component");
        self.pool.execute(move || {
            capture::polling::Poller::new(self.capture)
                .with_packet_cnt(1)
                .with_codec(self.codec)
                .poll();
        });
    }
}