use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local};
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use postgres::NoTls;
use serde_json::Value;
use crate::command::executor::Executor;

use tokio::runtime::Runtime;

pub struct InsertPacket {
    pub executor: Executor
}

impl InsertPacket {
    // add connecton here as a new parameter
    // https://docs.rs/elephantry/latest/elephantry/connection/struct.Connection.html
    pub fn insert(&self, frame_time: DateTime<Local>, src_addr: String, dst_addr: String, packet_json: Vec<u8>) {
        let json_value = Self::convert_to_value(packet_json).unwrap();
        let fut_res = async {
            let query = move |mut con: PooledConnection<PostgresConnectionManager<NoTls>>| -> Result<u64, postgres::Error>{
                con.execute(
                    "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
                    &[&frame_time, &src_addr, &dst_addr, &json_value],
                )
            };
            self.executor.execute(query).await
        };
        // It makes an overhead. Think about integrating tokio::runtime into the whole project
        let rt = Runtime::new().unwrap();
        rt.block_on(async{ 
            let result = fut_res.await;
            match result{
                Ok(_) => {}
                Err(error) => {
                    log::error!("{}", error)
                }
            }
        });
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}