use std::rc::Rc;
use net_proto_api::api::API;
use net_proto_api::envelope::envelope::Envelope;
use sqlx::Postgres;

pub mod network_graph;

pub trait ChartGenerator {
    fn generate_chart(transaction: &mut sqlx::Transaction<Postgres>, data: &Envelope)
        -> Result<Rc<dyn API>, String>;
}