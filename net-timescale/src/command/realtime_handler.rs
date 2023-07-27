use std::collections::HashSet;
use std::ops::DerefMut;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use async_std::task::block_on;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use chrono::{TimeZone, Utc};
use sqlx::Postgres;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use crate::command::executor::PoolWrapper;
use net_timescale_api::api::network_graph_request::NetworkGraphRequest;
use crate::internal_api::is_realtime::RealtimeRequestDTO;
use crate::persistence::network_graph;

pub struct IsRealtimeHandler {
    connections: Arc<RwLock<HashSet<i64>>>,
}

impl Default for IsRealtimeHandler {
    fn default() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashSet::default())),
        }
    }
}

impl IsRealtimeHandler {
    pub fn new(connections: Arc<RwLock<HashSet<i64>>>) -> Self {
        Self {
            connections,
        }
    }
}

impl Handler for IsRealtimeHandler {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        log::info!("in is_realtime handler");
        let data = receiver.recv();
        let real_req = RealtimeRequestDTO::decode(&data);
        let mut connections = block_on(self.connections.write());
        match connections.get(&real_req.get_connection_id()).is_some() {
            true => {
                connections.insert(real_req.get_connection_id());
                log::debug!("the connection {} is added to real-time", real_req.get_connection_id());
            },
            false => {
                connections.remove(&real_req.get_connection_id());
                log::debug!("the connection {} is removed from real-time", real_req.get_connection_id());
            }
        }
        log::info!("groups in is_realtime {:?}", connections);
    }
}