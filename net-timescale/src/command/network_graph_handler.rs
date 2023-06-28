use std::ops::DerefMut;
use std::sync::Arc;
use net_core::transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use chrono::{TimeZone, Utc};
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
use crate::command::executor::PoolWrapper;
use net_timescale_api::api::date_cut::DateCutDTO;
use crate::persistence::network_graph;

pub struct NetworkGraphHandler<T>
    where T: Sender + ?Sized,
{
    connection_pool: PoolWrapper,
    router: Arc<T>
}
impl<T> NetworkGraphHandler<T>
    where T: Sender + ?Sized,
{
    pub fn new(connection_pool: PoolWrapper, router: Arc<T>) -> Self {
        NetworkGraphHandler {
            connection_pool,
            router,
        }
    }
}
impl<T> Handler for NetworkGraphHandler<T>
    where T: Sender + ?Sized,
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let mut pooled_connection = self.connection_pool.get_connection();
        let date_cut = DateCutDTO::decode(data);
        let start_date = Utc.timestamp_millis_opt(date_cut.get_start_date_time()).unwrap();
        let end_date = Utc.timestamp_millis_opt(date_cut.get_end_date_time()).unwrap();
        let network_graph = network_graph::get_network_graph_by_date_cut(pooled_connection.deref_mut(),
            start_date, end_date
        );
        log::info!("got network graph {:?}", network_graph);
        let data = network_graph.encode();
        let data = Envelope::new("network_graph".to_string(), data).encode();
        self.router.send(data.as_slice());
    }
}