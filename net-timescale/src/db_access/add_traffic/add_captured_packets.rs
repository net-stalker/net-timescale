use std::sync::Arc;
use net_core::transport::sockets::Handler;
use postgres::types::ToSql;
use serde_json::Value;
use crate::command::executor::Executor;
use crate::command::dispatcher::PacketData;
use crate::db_access::{query_result, as_query, query};

pub struct AddCapturedPackets {
    pub executor: Executor
} 
pub struct UpdatedRows {
    pub rows: u64
}
impl query_result::QueryResultComponent for UpdatedRows {}
impl as_query::AsQuery for AddCapturedPackets {
    // for now we use QueryResult. TODO: make query services like a separeate components
    fn execute(&self, data: &[u8]) -> Result<query_result::QueryResult, &'static str> {
        let frame_data: PacketData = bincode::deserialize(&data).unwrap();
        let result = self.insert(frame_data);
        match result{
            Ok(rows_count) => { 
                log::info!("{} rows were updated", rows_count);
                query_result::QueryResult::builder().with_result(Arc::new(UpdatedRows {rows: rows_count})).build()
            }
            Err(error) => {
                log::error!("{}", error);
                query_result::QueryResult::builder().with_error("Couldn't add data into table").build()
            }
        }
    }
}  
struct AddPacketsQuery {
    pub raw_query: String,
    pub args: PacketData
}
impl AddPacketsQuery {
    pub fn new(args: PacketData) -> Self {
        AddPacketsQuery { 
            raw_query: "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)".to_owned(),
            args
        } 
    }
}
impl query::PostgresQuery for AddPacketsQuery {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let new_args: Vec<&(dyn ToSql + Sync)> = vec![
            &self.args.frame_time,
            &self.args.src_addr,
            &self.args.dst_addr,
            &self.args.json
        ];
        (
            self.raw_query.to_owned(),
            new_args
        )
    }
}
impl AddCapturedPackets {
    pub fn insert(&self, data: PacketData) -> Result<u64, postgres::Error> {
        let query = Box::new(AddPacketsQuery::new(data));
        self.executor.execute(query)
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}
impl Handler for AddCapturedPackets {
    fn handle(&self, receiver: &dyn net_core::transport::sockets::Receiver, sender: &dyn net_core::transport::sockets::Sender) {
        receiver.recv();
        log::info!("handle in AddCapturedPackets");
    }
}

#[cfg(test)]
mod tests{
    use crate::db_access::query::PostgresQuery;

    use super::*;
    #[test]
    fn test_add_packet_query(){
        let time_to_insert = "2020-01-01 00:00:00.000 +0000".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let src = "1".to_owned();
        let dst = "2".to_owned();
        let data = r#"{"test":"test"}"#;
        let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
        let packet = PacketData {
            frame_time: time_to_insert.clone(),
            src_addr: src.clone(),
            dst_addr: dst.clone(),
            json: json_data.clone() 
        };
        let query = Box::new(AddPacketsQuery::new(packet));
        let query_string_str = query.raw_query;
        assert_eq!(&query_string_str, "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
        assert_eq!(query.args.json, json_data);
        assert_eq!(query.args.src_addr, src);
        assert_eq!(query.args.dst_addr, dst); 
        assert_eq!(query.args.frame_time, time_to_insert);
    }
    #[test]
    fn test_add_packet_query_raw_parameters(){
        let time_to_insert = "2020-01-01 00:00:00.000 +0000".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let src = "1".to_owned();
        let dst = "2".to_owned();
        let data = r#"{"test":"test"}"#;
        let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
        let packet_1 = PacketData {
            frame_time: time_to_insert.clone(),
            src_addr: src.clone(),
            dst_addr: dst.clone(),
            json: json_data.clone()
        };
        let query = Box::new(AddPacketsQuery::new(packet_1));
        let (_query_string, args) = query.get_query();
        let new_args: Vec<&(dyn ToSql + Sync)> = vec![
            &time_to_insert,
            &src,
            &dst,
            &json_data
        ];
        assert_eq!(format!("{:?}", args), format!("{:?}", new_args))
    }
    #[test]
    fn test_bincode_for_packet_data(){
        let time_to_insert = "2020-01-01 00:00:00.000 +0000".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let src = "1".to_owned();
        let dst = "2".to_owned();
        let data = r#"{"test":"test"}"#;
        let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
        let packet = PacketData {
            frame_time: time_to_insert.clone(),
            src_addr: src.clone(),
            dst_addr: dst.clone(),
            json: json_data.clone()
        };
        let data = bincode::serialize(&packet).unwrap();
        let packet_copy: PacketData = bincode::deserialize(&data).unwrap(); 
        assert_eq!(packet.src_addr, packet_copy.src_addr);
        assert_eq!(packet.dst_addr, packet_copy.dst_addr);
        assert_eq!(packet.json, packet_copy.json);
        assert_eq!(packet.frame_time, packet_copy.frame_time);
    }
}