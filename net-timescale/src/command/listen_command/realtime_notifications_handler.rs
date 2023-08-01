use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, RwLock};
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

pub struct NetworkPacketHandler {
    pool: Arc<PoolWrapper<Postgres>>,
    // TODO: create wrapper for connections
    connections: Arc<RwLock<HashSet<i64>>>,
}
impl NetworkPacketHandler {
    pub fn new(pool: Arc<PoolWrapper<Postgres>>, connections: Arc<RwLock<HashSet<i64>>>) -> Self {
        NetworkPacketHandler {
            pool,
            connections,
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