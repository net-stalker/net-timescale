use std::sync::Arc;
use chrono::{Utc, DateTime, TimeZone};
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use r2d2::ManageConnection;
use serde_json::Value;
use crate::command::executor::Executor;
use net_timescale_api::api::{network_packet::NetworkPacketDTO};
use crate::persistence::sql_query::SqlQuery;
use super::sql_query::NetworkPacket;

fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
    serde_json::from_slice(&*packet_json)
}
pub struct NetworkPacketHandler<T>
where T: Sender + ?Sized,
{
    executor: Executor,
    result_receiver: Arc<T>
}
impl<T> NetworkPacketHandler<T>
where T: Sender + ?Sized,
{
    pub fn new(executor: Executor, result_receiver: Arc<T>) -> Self {
        NetworkPacketHandler {
            executor,
            result_receiver
        }
    }
}
impl<T> Handler for NetworkPacketHandler<T>
where T: Sender + ?Sized,
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let packet = NetworkPacketDTO::decode(data.to_owned());
        match self.executor.execute(NetworkPacket::new(packet).get_sql_query()) {
            Ok(rows_count) => {
                log::info!("{} rows were updated", rows_count);
            }
            Err(error) => {
                log::error!("{}", error);
            }
        };
        self.result_receiver.send("packets have been added".as_bytes());
    }
}

#[cfg(test)]
mod tests{
    // use postgres::types::ToSql;
    // use crate::persistence::sql_query::PostgresQuery;
    // use super::*;
    // #[test]
    // fn test_add_packet_query(){
    //     let time_to_insert = "2020-01-01 00:00:00.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
    //     let src = "1".to_owned();
    //     let dst = "2".to_owned();
    //     let data = r#"{"test":"test"}"#;
    //     let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
    //     let packet = NetworkPacketDTO::new(
    //         time_to_insert.timestamp_millis(),
    //         src.clone(),
    //         dst.clone(),
    //         data.as_bytes().to_owned()
    //     );
    //     let time = Utc.timestamp_millis_opt(packet.get_frame_time()).unwrap();
    //     let json = convert_to_value(packet.get_network_packet_data().to_owned()).unwrap();
    //     let src_addr = packet.get_src_addr().to_owned();
    //     let dst_addr = packet.get_dst_addr().to_owned();
    //     let query_struct = NetworkPacketQuery::new(&time, &src_addr, &dst_addr, &json);
    //
    //     let (query, params) = query_struct.get_query_params();
    //     assert_eq!(query, "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
    //
    //     assert_eq!(time, time_to_insert);
    //     assert_eq!(src, packet.get_src_addr());
    //     assert_eq!(dst, packet.get_dst_addr());
    //     assert_eq!(json_data, json);
    //
    //     let test_params: [&(dyn ToSql + Sync); 4] = [&time_to_insert, &src, &dst, &json_data];
    //     assert_eq!(format!("{:?}", params), format!("{:?}", &test_params));
    //
    // }
}