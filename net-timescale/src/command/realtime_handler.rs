use std::collections::HashSet;
use std::sync::RwLock;
use async_std::sync::Arc;
use futures::executor::block_on;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use sqlx::Postgres;
use crate::command::executor::PoolWrapper;
use crate::internal_api::is_realtime::RealtimeRequestDTO;

pub struct IsRealtimeHandler {
    connections: Arc<RwLock<HashSet<i64>>>,
    listen_handler: async_channel::Sender<Vec<u8>>,
}

impl IsRealtimeHandler {
    pub fn new(
        connections: Arc<RwLock<HashSet<i64>>>,
        listen_handler: async_channel::Sender<Vec<u8>>
    ) -> Self {
        Self {
            connections,
            listen_handler,
        }
    }
    fn activate_listening(&self) {
        let envelope = Envelope::new(
            "listen",
            "insert_channel".as_bytes()).encode();
        block_on(self.listen_handler.send(envelope))
            .expect("expected to be sent to listen handler");
    }
    fn deactivate_listening(&self) {
        let envelope = Envelope::new(
            "unlisten",
            "insert_channel".as_bytes()).encode();
        block_on(self.listen_handler.send(envelope))
            .expect("expected to be sent to listen handler");
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
                match real_req.is_subscribe() {
                    false => {
                        connections.remove(&real_req.get_connection_id());
                        log::debug!("the connection {} is removed from real-time", real_req.get_connection_id());
                        if connections.is_empty() {
                            self.deactivate_listening();
                        }
                    },
                    _ => {
                        log::debug!("the connection {} is already in real-time", real_req.get_connection_id());
                    }
                }
            },
            false => {
                match real_req.is_subscribe() {
                    true => {
                        connections.insert(real_req.get_connection_id());
                        log::debug!("the connection {} is added to real-time", real_req.get_connection_id());
                        if connections.len() == 1 {
                            self.activate_listening();
                        }
                    },
                    false => {
                        log::error!("wring request, there is no connection {} to be deleted", real_req.get_connection_id());
                    }
                }
            }
        }
        log::info!("connections in is_realtime {:?}", connections);
    }
}