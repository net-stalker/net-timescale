use postgres::types::ToSql;
use serde_json::Value;
use super::{as_query, query};
use crate::command::executor::Executor;
use crate::command::dispatcher::PacketData;

pub struct AddCapturedPackets {
    pub executor: Executor
}
impl as_query::AsQuery for AddCapturedPackets {
    fn execute(&self, data: &[u8]) {
        let frame_data: PacketData = bincode::deserialize(&data).unwrap();
        let result = self.insert(frame_data);
        match result{
            Ok(rows_count) => { 
                log::info!("{} rows were updated", rows_count);
            }
            Err(error) => {
                log::error!("{}", error);
            }
        };
    }
}  
struct AddPacketsQuery<'a> {
    pub raw_query: &'a str,
    pub args: [&'a (dyn ToSql + Sync); 4]
}
impl<'a> AddPacketsQuery<'a> {
    pub fn new(args: &'a PacketData) -> Self {
        AddPacketsQuery { 
            raw_query: "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
            args: [
                &args.frame_time,
                &args.src_addr,
                &args.dst_addr,
                &args.json
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
        let query = AddPacketsQuery::new(&data);
        self.executor.execute(&query)
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
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
        let query = Box::new(AddPacketsQuery::new(&packet));
        assert_eq!(query.raw_query, "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
    }
    #[test]
    fn test_add_packet_raw_parameters(){
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
        let query = Box::new(AddPacketsQuery::new(&packet_1));
        let (_query_string, args) = query.get_query_params();
        let new_args: Vec<&(dyn ToSql + Sync)> = vec![
            &time_to_insert,
            &src,
            &dst,
            &json_data
        ];
        println!("args {:?}", args);
        println!("args {:?}", new_args);
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