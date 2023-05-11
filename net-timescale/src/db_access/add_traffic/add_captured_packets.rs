use std::marker::PhantomData;
use std::sync::Arc;
use chrono::{Utc, DateTime, TimeZone};
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use postgres::types::ToSql;
use serde_json::Value;
use crate::command::executor::Executor;
use crate::db_access::query::PostgresQuery;
use crate::db_access::{query, query_factory};
use net_timescale_api::api::{network_packet::NetworkPacketDTO};

fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
    serde_json::from_slice(&*packet_json)
}
pub struct AddCapturedPackets<T, M>
where
    T: Sender + ?Sized,
    M: Executor<Q = dyn for<'b> query::PostgresQuery<'b>, R = Vec<postgres::Row>, E = postgres::Error> + ?Sized
{
    executor: Arc<M>,
    result_receiver: Arc<T>
}
impl<T, M> query_factory::QueryFactory for AddCapturedPackets<T, M>
where
    T: Sender + ?Sized,
    M: Executor<Q = dyn for<'b> query::PostgresQuery<'b>, R = Vec<postgres::Row>, E = postgres::Error> + ?Sized
{
    type Q = AddCapturedPackets<T, M>;
    type R = Arc<T>;
    type E = Arc<M>;
    fn create_query_handler(executor: Self::E, result_receiver: Self::R) -> Self::Q {
        AddCapturedPackets {
            executor,
            result_receiver
        }
    }
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
impl<T, M> AddCapturedPackets<T, M>
where
    T: Sender + ?Sized,
    M: Executor<Q = dyn for<'b> query::PostgresQuery<'b>, R = Vec<postgres::Row>, E = postgres::Error> + ?Sized
{
    pub fn insert(&self, data: NetworkPacketDTO) -> Result<u64, <M as Executor>::E> {
        let time = Utc.timestamp_millis_opt(data.get_frame_time()).unwrap();
        let json = convert_to_value(data.get_network_packet_data().to_owned()).unwrap();
        let src_addr = data.get_src_addr().to_owned();
        let dst_addr = data.get_dst_addr().to_owned();
        let query = AddPacketsQuery::new(&time, &src_addr, &dst_addr, &json);
        let dyn_query: &dyn PostgresQuery = &query;
        self.executor.execute(dyn_query)
    }
}
impl<T, M> Handler for AddCapturedPackets<T, M>
where
    T: Sender + ?Sized,
    M: Executor<Q = dyn for<'b> query::PostgresQuery<'b>, R = Vec<postgres::Row>, E = postgres::Error> + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        // ==============================
        // must be changed 
        let topic = "add_packet".as_bytes().to_owned();
        let packet = NetworkPacketDTO::decode(data[topic.len()..].to_owned());
        //==============================
        match self.insert(packet) {
            Ok(rows_count) => { 
                log::info!("{} rows were updated", rows_count);
            }
            Err(error) => {
                log::error!("{}", error);
            }
        };
        self.result_receiver.send("packets have been added".as_bytes().to_owned());
    }
}

#[cfg(test)]
mod tests{
    use postgres::NoTls;
    use r2d2_postgres::PostgresConnectionManager;

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
        let packet = NetworkPacketDTO::new(
            time_to_insert.timestamp_millis(),
            src.clone(),
            dst.clone(),
            data.as_bytes().to_owned()
        );
        let time = Utc.timestamp_millis_opt(packet.get_frame_time()).unwrap();
        let json = convert_to_value(packet.get_network_packet_data().to_owned()).unwrap();
        let src_addr = packet.get_src_addr().to_owned();
        let dst_addr = packet.get_dst_addr().to_owned();
        let query_struct = AddPacketsQuery::new(&time, &src_addr, &dst_addr, &json);

        let (query, params) = query_struct.get_query_params();
        assert_eq!(query, "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
        
        assert_eq!(time, time_to_insert);
        assert_eq!(src, packet.get_src_addr());
        assert_eq!(dst, packet.get_dst_addr());
        assert_eq!(json_data, json);

        let test_params: [&(dyn ToSql + Sync); 4] = [&time_to_insert, &src, &dst, &json_data];
        assert_eq!(format!("{:?}", params), format!("{:?}", &test_params));
        
    }
}