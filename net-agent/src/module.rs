use std::thread;
use std::thread::JoinHandle;

use log::info;
use shaku::{module, Component};

use net_core::capture;
use net_core::starter::starter::Starter;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;

use crate::codec::Codec;
use crate::command::dummy::DummyCommand;

module! {
    pub NetAgentModule {
        components = [Agent],
        providers = []
    }
}

#[derive(Component)]
#[shaku(interface = Starter)]
pub struct Agent;

impl Starter for Agent {
    fn start(&self) -> JoinHandle<()> {
        info!("Start module");
        // let config = ConfigManager {
        //     application_name: "net-agent",
        //     file_loader: Box::new(ConfigFile) as Box<dyn FileReader>,
        // }
        // .load();
        // let config: String = (&self.config.dealer.endpoint).parse().unwrap();
        let client = ConnectorNNG::builder()
            .with_endpoint("tcp://0.0.0.0:5555".to_string())
            .with_proto(Proto::Req)
            .with_handler(DummyCommand)
            .build()
            .connect()
            .into_inner();
        let client_clone = client.clone();

        thread::spawn(move || {
            let capture = pcap::Capture::from_device("en0")
                .unwrap()
                // .promisc(true)
                // .snaplen(65535)
                .buffer_size(1000)
                .open()
                .unwrap();

            let codec = Codec::new(client_clone);
            capture::polling::Poller::new(capture)
                .with_packet_cnt(1)
                .with_codec(codec)
                .poll();
        });

        thread::spawn(move || {
            Poller::new()
                .add(client)
                .poll();
        })
    }
}
