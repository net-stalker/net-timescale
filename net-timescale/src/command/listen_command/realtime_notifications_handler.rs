use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use futures::executor::block_on;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;
use sqlx::Postgres;
use net_proto_api::envelope::envelope::Envelope;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use crate::command::executor::PoolWrapper;
use crate::internal_api::notification::NotificationDTO;

pub struct RealtimeNotificationHandler<S>
where S: Sender
{
    pool: Arc<PoolWrapper<Postgres>>,
    router: Arc<S>,
    // TODO: create wrapper for connections
    connections: Arc<RwLock<HashSet<i64>>>,
}
impl<S> RealtimeNotificationHandler<S>
where S: Sender
{
    pub fn new(pool: Arc<PoolWrapper<Postgres>>,
               router: Arc<S>,
               connections: Arc<RwLock<HashSet<i64>>>,
    ) -> Self {
        RealtimeNotificationHandler {
            pool,
            router,
            connections,
        }
    }
    async fn handle_insert_channel(&self) -> NetworkGraphDTO {
        // 1) find minimum index from the table
        // 2) query graph by index
        // 3) update indexes
        // 4) send to router
        let mut db_connection = self.pool.get_connection().await;
        let mut transction = db_connection.begin().await.unwrap();

        todo!();
    }
}
impl<S> Handler for RealtimeNotificationHandler<S>
where S: Sender
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(data.as_slice());
        let notification = NotificationDTO::decode(envelope.get_data());
        // index must be present in the table here
        match notification.get_channel() {
            "insert_channel" => {
                log::info!("got notification from {}", notification.get_channel());
                // 1) find minimum index from the table
                let graph = block_on(self.handle_insert_channel()).encode();
                let envelope = Envelope::new("network_graph", graph.as_slice()).encode();
                self.router.send(envelope.as_slice());
            },
            _ => {
                log::error!("wrong channel {}", notification.get_channel());
            }
        }
    }
}