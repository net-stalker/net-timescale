use chrono::{Utc, DateTime, TimeZone};
use net_core::transport::sockets::Handler;
use nng::Socket;
use postgres::types::ToSql;
use serde_json::Value;
use crate::db_access::query;
use crate::command::executor::Executor;
use super::packet_data::PacketData;

pub struct AddCapturedPackets {
    pub executor: Executor,
    pub sender_back: Socket
}  
struct AddPacketsQuery<'a> {
    pub raw_query: &'a str,
    pub args: [&'a (dyn ToSql + Sync); 4]
}
impl<'a> AddPacketsQuery<'a> {
    pub fn new(time: &'a DateTime<Utc>, src_addr: &'a String, dst_addr: &'a String, json_data: &'a serde_json::Value) -> Self {
        AddPacketsQuery { 
            raw_query: "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
            args: [
                time,
                src_addr,
                dst_addr,
                json_data
            ]
        } 
    }
}
impl<'a> query::PostgresQuery<'a> for AddPacketsQuery<'a> {
    fn get_query_params(&self) -> (&'a str, &[&'a(dyn postgres::types::ToSql + Sync)]) {
        (self.raw_query, &self.args)
    }
}
impl AddCapturedPackets {
    pub fn insert(&self, data: PacketData) -> Result<u64, postgres::Error> {
        let time = Utc.timestamp_millis_opt(data.frame_time).unwrap();
        let json = AddCapturedPackets::convert_to_value(data.binary_json).unwrap();
        let query = AddPacketsQuery::new(&time, &data.src_addr, &data.dst_addr, &json);
        self.executor.execute(&query)
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}
impl Handler for AddCapturedPackets {
    fn handle(&self, receiver: &dyn net_core::transport::sockets::Receiver, _sender: &dyn net_core::transport::sockets::Sender) {
        let data = receiver.recv();
        log::info!("Data in add_traffic: {:?}", data);
        // ==============================
        // must be changed 
        let topic = "add_packet".as_bytes().to_owned();
        let frame_data: PacketData = bincode::deserialize(&data[topic.len()..]).unwrap();
        //==============================
        match self.insert(frame_data) {
            Ok(rows_count) => { 
                log::info!("{} rows were updated", rows_count);
            }
            Err(error) => {
                log::error!("{}", error);
            }
        };
        self.sender_back.send("Traffic is here".as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests{
    use crate::db_access::query::PostgresQuery;

    use super::*;
    #[test]
    fn test_add_packet_query_raw_params(){
        let time_to_insert = "2020-01-01 00:00:00.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let src = "1".to_owned();
        let dst = "2".to_owned();
        let data = r#"{"test":"test"}"#;
        let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
        let query_struct = AddPacketsQuery::new(&time_to_insert, &src, &dst, &json_data);
        
        let (query, params) = query_struct.get_query_params();
        assert_eq!(query, "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
        
        let test_params: [&(dyn ToSql + Sync); 4] = [&time_to_insert, &src, &dst, &json_data];
        assert_eq!(format!("{:?}", params), format!("{:?}", &test_params));
    }
    #[test]
    fn test_add_packet_query(){
        let time_to_insert = "2020-01-01 00:00:00.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let src = "1".to_owned();
        let dst = "2".to_owned();
        let data = r#"{"test":"test"}"#;
        let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
        let packet = PacketData {
            frame_time: time_to_insert.timestamp_millis(),
            src_addr: src.clone(),
            dst_addr: dst.clone(),
            binary_json: data.as_bytes().to_owned()
        };
        let time = Utc.timestamp_millis_opt(packet.frame_time).unwrap();
        let json = AddCapturedPackets::convert_to_value(packet.binary_json).unwrap();
        let query_struct = AddPacketsQuery::new(&time, &packet.src_addr, &packet.dst_addr, &json);

        let (query, params) = query_struct.get_query_params();
        assert_eq!(query, "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
        
        assert_eq!(time, time_to_insert);
        assert_eq!(src, packet.src_addr);
        assert_eq!(dst, packet.dst_addr);
        assert_eq!(json_data, json);

        let test_params: [&(dyn ToSql + Sync); 4] = [&time_to_insert, &src, &dst, &json_data];
        assert_eq!(format!("{:?}", params), format!("{:?}", &test_params));
        
    }
}