use std::ops::DerefMut;
use std::sync::Arc;
use futures::executor::block_on;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use serde_json::Value;
use sqlx::Postgres;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use crate::command::executor::PoolWrapper;
use net_timescale_api::api::{network_packet::NetworkPacketDTO};
use crate::repository::{
    network_packet::{
        NetworkPacket,
        self
    },
};

fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
    serde_json::from_slice(&*packet_json)
}
pub struct NetworkPacketHandler {
    pool: Arc<PoolWrapper<Postgres>>,
    notify_channel: String,
}
impl NetworkPacketHandler {
    pub fn new(executor: Arc<PoolWrapper<Postgres>>, notify_channel: String) -> Self {
        NetworkPacketHandler {
            pool: executor,
            notify_channel,
        }
    }
}
impl Handler for NetworkPacketHandler {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(data.as_slice());
        let packet = NetworkPacketDTO::decode(envelope.get_data());
        let mut pooled_connection = block_on(self.pool.get_connection());
        match block_on(network_packet::insert_network_packet(pooled_connection, packet.into())) {
            Ok(rows_count) => {
                log::info!("{} rows were affected", rows_count.rows_affected());
            }
            Err(error) => {
                log::error!("{}", error);
            }
        };
    }
}