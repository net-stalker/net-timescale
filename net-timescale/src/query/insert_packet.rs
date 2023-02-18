use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local};
use postgres::Client;
use serde_json::Value;

pub struct InsertPacket {
    pub conn: Arc<Mutex<Client>>,
}

impl InsertPacket {
    pub fn insert(&self, frame_time: DateTime<Local>, packet_json: Vec<u8>) {
        let json_value = Self::convert_to_value(packet_json).unwrap();

        self.conn.lock().unwrap()
            .execute(
                "INSERT INTO CAPTURED_TRAFFIC (frame_time, binary_data) VALUES ($1, $2)",
                &[&frame_time, &json_value],
            ).unwrap();
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}

#[cfg(test)]
mod tests {
    use postgres::NoTls;

    use net_core::file::files::{Files, Reader};
    use net_core::json_pcap_parser::JsonPcapParser;

    use super::*;

    #[test]
    fn expected_insert_packet() {
        let mut conn = Client::connect("postgres://postgres:PsWDgxZb@localhost", NoTls).unwrap();
        let insert_packet = InsertPacket { conn: Arc::new(Mutex::new(conn)) };


        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/captures/arp_layer_extracted.json");
        let frame_time = JsonPcapParser::find_frame_time(Files::read(path));
        let json_bytes = Files::read(path);

        insert_packet.insert(frame_time, json_bytes);
    }
}