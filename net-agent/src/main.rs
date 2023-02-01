use std::thread;

use net_core::config::{ConfigManager, ConfigSpec, ConfigFile, FileReader};
use net_core::capture::pcapture::{capture_packages, create_global_header};
use net_core::transport::connector_nng::{ConnectorNng, Proto};
use net_core::transport::context::{ContextBuilder};
use net_core::transport::polling::Poller;
use net_core::transport::sockets::{Handler, Receiver, Sender};

fn main() {
    let config = ConfigManager { application_name: "net-agent", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load();
    if !config.dealer.enable {
        println!("Dealer is disabled!");
        return;
    }

    struct ClientCommand;
    impl Handler for ClientCommand {
        fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {}
    }

    let client = ConnectorNng::builder()
        .with_endpoint(config.dealer.endpoint)
        .with_proto(Proto::Req)
        .with_handler(ClientCommand)
        .build()
        .connect()
        .into_inner();
    let arc = client.clone();

    let client_handle = thread::spawn(move || {
        let global_header = create_global_header();
        println!("Global Header {}", global_header);
        //Send first packet as Global Header of pcap file
        arc.send(global_header.as_bytes());
        // client.send(global_header.as_bytes());

        capture_packages(config.data, |_cnt, packet| {
            //Send pcap packet header + packet payload
            // client.send(packet.as_bytes())
            arc.send(packet.as_bytes())
        });
    });

    let poller = thread::spawn(move || {
        Poller::new()
            .add(client)
            .poll();
    });
    poller.join().unwrap();
    client_handle.join().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_load_configuration() {
        // let config = ConfigManager { file_loader: FileLoader }.load_config();

        // println!("{}", config);
    }
}