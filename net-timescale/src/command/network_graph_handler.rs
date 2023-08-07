use std::ops::DerefMut;
use std::sync::Arc;
use async_std::task::block_on;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use chrono::{TimeZone, Utc};
use sqlx::Postgres;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use crate::command::executor::PoolWrapper;
use net_timescale_api::api::network_graph_request::NetworkGraphRequestDTO;
use crate::internal_api::is_realtime::RealtimeRequestDTO;
use crate::persistence::network_graph;

pub struct NetworkGraphHandler<T, S>
where
    T: Sender + ?Sized,
    S: Sender + ?Sized,
{
    connection_pool: Arc<PoolWrapper<Postgres>>,
    router: Arc<T>,
    is_realtime_handler: Arc<S>,
}
impl<T, S> NetworkGraphHandler<T, S>
where
    T: Sender + ?Sized,
    S: Sender + ?Sized,
{
    pub fn new(
        connection_pool: Arc<PoolWrapper<Postgres>>,
        router: Arc<T>,
        is_realtime_handler: Arc<S>,
    ) -> Self {
        NetworkGraphHandler {
            connection_pool,
            router,
            is_realtime_handler,
        }
    }
}
impl<T, S> Handler for NetworkGraphHandler<T, S>
where
    T: Sender + ?Sized,
    S: Sender + ?Sized,
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(&data);

        let pooled_connection = block_on(self.connection_pool.get_connection());
        // TODO: add connection_id as a field in DTO structures
        const MOCK_CONNECTION_ID: i64 = 90;
        let graph_request = NetworkGraphRequestDTO::decode(envelope.get_data());

        let end_date_time = graph_request.get_end_date_time();
        let subscription = graph_request.is_subscribe();
        // TODO: there is a need to return result
        let network_graph = block_on(network_graph::reply_network_graph_request(
            pooled_connection,
            graph_request.into(),
            MOCK_CONNECTION_ID
        ));

        log::info!("got network graph {:?}", network_graph);
        let data = network_graph.encode();
        let data = Envelope::new("network_graph", &data).encode();
        if end_date_time == 0 {
            self.is_realtime_handler.send(RealtimeRequestDTO::new(
                MOCK_CONNECTION_ID,
                subscription
            ).encode().as_slice());
        }
        self.router.send(data.as_slice());
    }
}