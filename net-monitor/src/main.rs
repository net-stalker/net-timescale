use std::thread;
use rand::{Rng, thread_rng};

use net_core::config::{ConfigManager, ConfigSpec, ConfigFile, FileReader};
use net_core::capture::pcapture::{capture_packages, create_global_header};
use net_core::transport::connector::{ConnectorBuilder, Sender, Poller};
use net_core::transport::context::{ContextBuilder};

fn main() {
    let config = ConfigManager { application_name: "net-monitor", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load();
    if !config.dealer.enable {
        println!("Dealer is disabled!");
        return;
    }

    let context = ContextBuilder::new().build();
    let client = ConnectorBuilder::new()
        .context(context)
        .xtype(zmq::DEALER)
        .endpoint(config.dealer.endpoint)
        .handler(|data| {})
        .build()
        .connect();

    let global_header = create_global_header();
    println!("Global Header {}", global_header);
    client.send(global_header.as_bytes());

    capture_packages(config.data, |_cnt, packet| client.send(packet.as_bytes()));

    thread::spawn(move || client.poll())
        .join()
        .unwrap();
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