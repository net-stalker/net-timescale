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
                log::error!("{}", error)
            }
        }
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}