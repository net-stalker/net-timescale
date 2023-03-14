use std::sync::{Arc, Mutex};
use std::thread;

use chrono::{DateTime, Local};
use log::{debug, error, info};
use postgres::Client;
use postgres::fallible_iterator::FallibleIterator;
use serde_json::Value;

pub struct QueryPacket {
    pub client: Arc<Mutex<Client>>,
}

//FIXME Created for test. Should be refactored in the future
impl QueryPacket {
    pub fn subscribe(&self) {
        // let json_value = Self::convert_to_value(packet_json).unwrap();

        // let result = self.client.lock().unwrap()
        //     .execute(
        //         "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
        //         &[&frame_time, &src_addr, &dst_addr, &json_value],
        //     );

        // match result {
        //     Ok(_) => {}
        //     Err(error) => {
        //         error!("{}", error)
        //     }
        // }
        let arc = self.client.clone();
        thread::spawn(move || {
            // Listen for events on channel 'myevent'.
            let mut conn = arc.lock().unwrap();
            conn.execute("LISTEN core_db_event", &[]).expect("Could not send LISTEN");
            let mut notifications = conn.notifications();
            let mut it = notifications.blocking_iter();

            loop {
                info!("Waiting for notifications...");
                let a = it.next();
                match a {
                    Ok(Some(b)) => {
                        debug!("{:?}", b);
                    }
                    Err(e) => error!("Got error {:?}", e),
                    _ => panic!("Unexpected operation!!!")
                }
            }
        });
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}

#[cfg(test)]
mod tests {
    use postgres::NoTls;

    use net_core::file::files::Files;
    use net_core::jsons::json_pcap_parser::JsonPcapParser;

    use super::*;

    #[test]
    fn expected_insert_packet() {
        let mut client = Client::connect("postgres://postgres:PsWDgxZb@localhost", NoTls).unwrap();
        let insert_packet = QueryPacket { client: Arc::new(Mutex::new(client)) };


        // let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/captures/arp_layer_extracted.json");
        // let json_bytes = Files::read(path);
        // let result = JsonPcapParser::find_frame_time(json_bytes);

        // insert_packet.insert(result.0, , , result.1);
    }
}