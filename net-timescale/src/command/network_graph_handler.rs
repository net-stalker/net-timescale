use std::sync::Arc;
use async_std::task::block_on;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use sqlx::Postgres;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use crate::command::executor::PoolWrapper;
use net_timescale_api::api::network_graph_request::NetworkGraphRequestDTO;
use crate::internal_api::realtime_request::RealtimeRequestDTO;
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
        let network_graph_request = NetworkGraphRequestDTO::decode(envelope.get_data());
        let pooled_connection = block_on(self.connection_pool.get_connection());
        let network_graph = block_on(network_graph::get_network_graph_by_date_cut(pooled_connection, &envelope));

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
        if network_graph_request.get_end_date_time() == 0 {
            let mock_connection_id = 90;
            let realtime_request = RealtimeRequestDTO::new(mock_connection_id).encode();
            let enveloped_realtime_request = Envelope::new(group_id, agent_id, "realtime_request", &realtime_request); 
            self.is_realtime_handler.send(&enveloped_realtime_request.encode());
        }
        self.router.send(data.as_slice());
    }
}