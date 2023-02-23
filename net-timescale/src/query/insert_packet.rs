use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local};
use postgres::Client;
use serde_json::Value;

pub struct InsertPacket {
    pub client: Arc<Mutex<Client>>,
}

impl InsertPacket {
    pub fn insert(&self, frame_time: DateTime<Local>, src_addr: String, dst_addr: String, packet_json: Vec<u8>) {
        let json_value = Self::convert_to_value(packet_json).unwrap();

        let result = self.client.lock().unwrap()
            .execute(
                "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
                &[&frame_time, &src_addr, &dst_addr, &json_value],
            );

        match result {
            Ok(_) => {}
            Err(error) => {
                println!("{}", error)
            }
        }
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}

#[cfg(test)]
mod tests {
    use postgres::NoTls;

    use net_core::file::files::{Files, Reader};
    use net_core::jsons::json_pcap_parser::JsonPcapParser;

    use super::*;

    #[test]
    fn expected_insert_packet() {
        let mut client = Client::connect("postgres://postgres:PsWDgxZb@localhost", NoTls).unwrap();
        let insert_packet = InsertPacket { client: Arc::new(Mutex::new(client)) };


        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/captures/arp_layer_extracted.json");
        let json_bytes = Files::read(path);
        let result = JsonPcapParser::find_frame_time(json_bytes);

        // insert_packet.insert(result.0, , , result.1);
    }
}