use std::sync::Arc;
use chrono::{Utc, TimeZone};
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_timescale_api::Decoder;
use serde_json::Value;
use crate::{
    persistence::query_factory,
    command::executor::Executor
};
use net_timescale_api::api::network_packet::NetworkPacket;
use super::network_packet_query::NetworkPacketQuery;

pub struct NetworkPacketHandler<T>
where T: Sender + ?Sized
{
    executor: Executor,
    result_receiver: Arc<T>
}
impl<T> query_factory::QueryFactory for NetworkPacketHandler<T>
where T: Sender + ?Sized
{
    type Q = NetworkPacketHandler<T>;
    type R = Arc<T>;
    fn create_query_handler(executor: Executor, result_receiver: Self::R) -> Self::Q {
        NetworkPacketHandler {
            executor,
            result_receiver
        }
    }
} 
impl<T> NetworkPacketHandler<T>
where T: Sender + ?Sized
{
    pub fn insert(&self, data: NetworkPacket) -> Result<u64, postgres::Error> {
        let time = Utc.timestamp_millis_opt(data.get_frame_time()).unwrap();
        let json = NetworkPacketHandler::<T>::convert_to_value(data.get_network_packet_data().to_owned()).unwrap();
        let src_addr = data.get_src_addr().to_owned();
        let dst_addr = data.get_dst_addr().to_owned();
        let query = NetworkPacketQuery::new(&time, &src_addr, &dst_addr, &json);
        self.executor.execute(&query)
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}
impl<T> Handler for NetworkPacketHandler<T>
where T: Sender + ?Sized
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        // ==============================
        // must be changed 
        let topic = "add_packet".as_bytes().to_owned();
        let packet = NetworkPacket::decode(data[topic.len()..].to_owned());
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
    use super::*;
    #[test]
    fn test_add_packet_query(){
        let time = "2020-01-01 00:00:00.000 UTC".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
        let src = "1".to_owned();
        let dst = "2".to_owned();
        let data = r#"{"test":"test"}"#;
        let json_data: serde_json::Value = serde_json::from_str(data).unwrap();
        let packet = NetworkPacket::new(
            time.timestamp_millis(),
            src.clone(),
            dst.clone(),
            data.as_bytes().to_owned()
        );
        let time_from_packet = Utc.timestamp_millis_opt(packet.get_frame_time()).unwrap();
        let json_from_packet = NetworkPacketHandler::<dyn Sender>::convert_to_value(packet.get_network_packet_data().to_owned()).unwrap();
        let src_from_packet = packet.get_src_addr().to_owned();
        let dst_from_packet = packet.get_dst_addr().to_owned();
        
        assert_eq!(time, time_from_packet);
        assert_eq!(src, src_from_packet);
        assert_eq!(dst, dst_from_packet);
        assert_eq!(json_data, json_from_packet);
    }
}