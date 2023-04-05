use std::sync::Arc;
use chrono::{DateTime, Local};
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use postgres::NoTls;
use serde_json::Value;
use super::request::Request;
use super::query_result::{QueryResult, ResultComponent};
use crate::command::executor::Executor;
use crate::command::dispatcher::FrameData;

// TODO: move UpdatedRows into dispatcher.rs
pub struct UpdatedRows{
    pub rows: u64
}
impl ResultComponent for UpdatedRows {}
pub struct InsertPacket {
    // executor is thread safe by itself
    pub executor: Executor
}

impl Request for InsertPacket{
    fn execute(&self, data: &[u8]) -> Result<QueryResult, &'static str> {
        let frame_data: FrameData = bincode::deserialize(&data).unwrap();
        let result = self.insert(
            frame_data.frame_time.parse::<DateTime<Local>>().unwrap(),
            frame_data.src_addr,
            frame_data.dst_addr,
            frame_data.binary_json);
        match result{
            Ok(rows_count) => {
                // TODO: move logging into dispatcher 
                log::info!("{} rows were updated", rows_count);
                return QueryResult::builder().with_result(Arc::new(UpdatedRows {rows: rows_count})).build();
            }
            Err(error) => {
                // TODO: move logging into dispatcher
                log::error!("{}", error);
                return QueryResult::builder().build();
            }
        }
    }
}

impl InsertPacket {
    pub fn insert(&self, frame_time: DateTime<Local>, src_addr: String, dst_addr: String, packet_json: Vec<u8>) -> Result<u64, postgres::Error> {
        let json_value = Self::convert_to_value(packet_json).unwrap();
        let query = move |mut con: PooledConnection<PostgresConnectionManager<NoTls>>| -> Result<u64, postgres::Error> {
            con.execute(
                "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
                &[&frame_time, &src_addr, &dst_addr, &json_value],
            )
        };
        self.executor.execute_query(query)
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}