use std::collections::HashSet;
use std::sync::RwLock;
use async_std::sync::Arc;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use sqlx::Postgres;
use crate::command::executor::PoolWrapper;
use crate::internal_api::is_realtime::RealtimeRequestDTO;

pub struct IsRealtimeHandler {
    connection_pool: Arc<PoolWrapper<Postgres>>,
    connections: Arc<RwLock<HashSet<i64>>>,
    listen_handler: async_channel::Sender<Vec<u8>>,
}

impl IsRealtimeHandler {
    pub fn new(
        connection_pool: Arc<PoolWrapper<Postgres>>,
        connections: Arc<RwLock<HashSet<i64>>>,
        // listen_handler: async_channel::Sender<Vec<u8>>
    ) -> Self {
        Self {
            connection_pool,
            connections,
            listen_handler: async_channel::unbounded().0,
        }
    }
}

impl Handler for IsRealtimeHandler {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        log::info!("in is_realtime handler");
        let data = receiver.recv();
        let real_req = RealtimeRequestDTO::decode(&data);
        let mut connections = self.connections.write().unwrap();
        match connections.get(&real_req.get_connection_id()).is_some() {
            true => {
                connections.remove(&real_req.get_connection_id());
                log::debug!("the connection {} is removed from real-time", real_req.get_connection_id());
            },
            false => {
                connections.insert(real_req.get_connection_id());
                log::debug!("the connection {} is added to real-time", real_req.get_connection_id());
            }
        }
        log::info!("groups in is_realtime {:?}", connections);
    }
}