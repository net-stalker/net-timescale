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
        let graph_request = NetworkGraphRequestDTO::decode(envelope.get_data());
        let start_date = Utc.timestamp_millis_opt(graph_request.get_start_date_time()).unwrap();
        let end_date = Utc.timestamp_millis_opt(graph_request.get_end_date_time()).unwrap();
        let network_graph = block_on(network_graph::get_network_graph_by_date_cut(pooled_connection,
            start_date, end_date
        ));
        log::info!("got network graph {:?}", network_graph);
        let agent_id = match envelope.get_agent_id() {
            Ok(id) => Some(id),
            Err(_) => None
        };
        let group_id = match envelope.get_group_id() {
            Ok(id) => Some(id),
            Err(_) => None
        };
        
        let data = network_graph.encode();
        let data = Envelope::new(
            group_id,
            agent_id,
            "network_graph",
            &data).encode();
        if graph_request.get_end_date_time() == 0 {
            let mock_connection_id = 90;
            self.is_realtime_handler.send(RealtimeRequestDTO::new(mock_connection_id).encode().as_slice());
        }
        self.router.send(data.as_slice());
    }
}