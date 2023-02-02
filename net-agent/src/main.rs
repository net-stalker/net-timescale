use std::thread;

use net_core::config::{ConfigManager, ConfigSpec, ConfigFile, FileReader};
use net_core::capture::pcapture::{capture_packages, create_global_header};
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::context::{ContextBuilder};
use net_core::transport::polling::Poller;
use net_core::transport::sockets::Sender;
use net_monitor::client_command::ClientCommand;

fn main() {
    let config = ConfigManager { application_name: "net-agent", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load();
    if !config.dealer.enable {
        println!("Dealer is disabled!");
        return;
    }

    let client = ConnectorNNG::builder()
        .with_endpoint(config.dealer.endpoint)
        .with_proto(Proto::Req)
        .with_handler(ClientCommand)
        .build()
        .connect()
        .into_inner();
    let client_clone = client.clone();

    thread::spawn(move || {
        //TODO should be moved to standalone command
        let global_header = create_global_header();
        // println!("Global Header {}", global_header);
        //Send first packet as Global Header of pcap file
        // client_clone.send(global_header.as_bytes());
        // client.send(global_header.as_bytes());

        capture_packages(config.data, |_cnt, packet| {
            //Send pcap packet header + packet payload
            let mut buf = global_header.as_bytes();
            buf.append(&mut packet.as_bytes());
            // client_clone.send(packet.as_bytes())
            client_clone.send(buf)
        });
    });

    thread::spawn(move || {
        Poller::new()
            .add(client)
            .poll();
    }).join().unwrap();
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