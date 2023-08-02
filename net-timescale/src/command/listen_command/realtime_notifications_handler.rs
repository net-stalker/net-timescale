use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use sqlx::Postgres;
use net_proto_api::envelope::envelope::Envelope;
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
                // find minimum index in the table
                // query data from captured_traffic
                // update indexes in the table with indexes
                // form graph
                // send graph to router. TODO: think about optimizing this step
            },
            _ => {
                log::error!("wrong channel {}", notification.get_channel());
            }
        }
    }
}