use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use futures::executor::block_on;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;
use sqlx::{Encode, Postgres};
use net_proto_api::envelope::envelope::Envelope;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use crate::command::executor::PoolWrapper;
use crate::internal_api::notification::NotificationDTO;
use crate::repository::realtime_client;
use crate::repository::captured_traffic;
use crate::persistence::network_graph;

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
    async fn handle_insert_channel(&self) -> Option<NetworkGraphDTO>{
        let connections = self.connections.read().unwrap();
        let db_connection = self.pool.get_connection().await;

        let mut transaction = db_connection.begin().await.unwrap();
        let index = match realtime_client::get_min_index(&mut transaction).await {
            Ok(index) => {
                index
            },
            Err(err) => {
                log::error!("couldn't query index: {}", err);
                return None;
            },
        };

        let graph = match network_graph::get_network_graph_by_index(
            &mut transaction,
            index
        ).await {
            Ok(graph) => {
                graph
            },
            Err(err) => {
                log::error!("couldn't query network graph: {}", err);
                return None;
            }
        };

        let new_index = match captured_traffic::get_max_id(
            &mut transaction
        ).await {
            Ok(new_index) => {
                new_index
            },
            Err(err) => {
                log::error!("couldn't query a new index: {}", err);
                return None;
            }
        };
        for connection in connections.iter() {
            if let Err(err) = realtime_client::update_last_index(
                &mut transaction,
                *connection,
                new_index
            ).await {
                log::error!("couldn't update index of {}: {}", *connection, err);
                return None;
            }
        }
        transaction.commit().await.expect("successful commit is expected");

        Some(graph)
    }
}
impl<S> Handler for RealtimeNotificationHandler<S>
where S: Sender
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        // NOTABLE: this handler can be a dispatcher in future if we have multiple channels to handle
        // as we have a single channel in DB that can be listened we choose simplicity over complexity
        let data = receiver.recv();
        let envelope = Envelope::decode(data.as_slice());
        let notification = NotificationDTO::decode(envelope.get_data());
        // index must be present in the table here
        match notification.get_channel() {
            "insert_channel" => {
                log::info!("got notification from {}", notification.get_channel());
                let graph: Vec<u8> = if let Some(graph) = block_on(self.handle_insert_channel()) {
                    graph.encode()
                } else { return; };

                let envelope = Envelope::new("network_graph", graph.as_slice()).encode();
                self.router.send(envelope.as_slice());
            },
            _ => {
                log::error!("wrong channel {}", notification.get_channel());
            }
        }
    }
}