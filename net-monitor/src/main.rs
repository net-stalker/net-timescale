use rand::{Rng, thread_rng};

use net_commons::config::{ConfigManager, ConfigSpec, ConfigFile, FileReader};
use net_commons::pcapture::{capture_packages, create_global_header};

fn main() {
    let config = ConfigManager { application_name: "net-monitor", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load();
    if !config.dealer.enable {
        println!("Dealer is disabled!");
        return;
    }

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::DEALER).unwrap();
    let mut rng = thread_rng();
    let identity = format!("{:04X}-{:04X}", rng.gen::<u16>(), rng.gen::<u16>());
    socket
        .set_identity(identity.as_bytes())
        .expect("failed setting client id");

    socket
        .connect(&config.dealer.endpoint)
        .expect("failed connecting client");

    let global_header = create_global_header();
    println!("Global Header {}", global_header);
    socket
        .send(global_header.as_bytes(), 0)
        .expect("client failed sending request");

    capture_packages(
        config.data,
        |_cnt, packet| {
            socket
                .send(packet.as_bytes(), 0)
                .expect("client failed sending request");
        });

    loop {
        let mut items = [socket.as_poll_item(zmq::POLLIN)];

        let rc = zmq::poll(&mut items, -1).unwrap();
        if rc == -1 {
            break;
        }

        if items[0].is_readable() {
            let msg = socket
                .recv_string(0)
                .expect("client failed receiving response");
            println!("{:?}", msg);
        }
    }
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