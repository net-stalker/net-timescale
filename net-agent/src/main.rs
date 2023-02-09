
use std::thread;

use net_agent::codec::Codec;
use net_agent::command::dummy::DummyCommand;
use net_core::capture;



use net_core::config::{ConfigFile, ConfigManager, ConfigSpec, FileReader};
use net_core::transport::connector_nng::{ConnectorNNG, Proto};

use net_core::transport::polling::Poller;


fn main() {
    let config = ConfigManager { application_name: "net-agent", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load();
    if !config.dealer.enable {
        println!("Dealer is disabled!");
        return;
    }

    let client = ConnectorNNG::builder()
        .with_endpoint(config.dealer.endpoint)
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
    }).join().unwrap();
}


#[cfg(test)]
mod tests {
    

    #[test]
    fn expected_load_configuration() {
        // let config = ConfigManager { file_loader: FileLoader }.load_config();

        // println!("{}", config);
    }
}