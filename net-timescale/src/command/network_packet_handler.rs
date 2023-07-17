use std::ops::DerefMut;
use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use serde_json::Value;
use crate::command::executor::PoolWrapper;
use net_timescale_api::api::{network_packet::NetworkPacketDTO};
use crate::repository::network_packet::{NetworkPacket, self};

fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
    serde_json::from_slice(&*packet_json)
}
pub struct NetworkPacketHandler<T>
    where T: Sender + ?Sized,
{
    pool: PoolWrapper,
    result_receiver: Arc<T>
}
impl<T> NetworkPacketHandler<T>
    where T: Sender + ?Sized,
{
    pub fn new(executor: PoolWrapper, result_receiver: Arc<T>) -> Self {
        NetworkPacketHandler {
            pool: executor,
            result_receiver
        }
    }
}
impl<T> Handler for NetworkPacketHandler<T>
    where T: Sender + ?Sized,
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let packet = NetworkPacketDTO::decode(&data);
        let mut pooled_connection = self.pool.get_connection();
        match network_packet::insert_network_packet(pooled_connection.deref_mut(), packet.into()) {
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