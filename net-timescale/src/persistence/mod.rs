use async_std::task::block_on;
use std::rc::Rc;

use sqlx::Postgres;
use sqlx::Transaction;

use net_proto_api::api::API;
use net_proto_api::envelope::envelope::Envelope;


#[async_trait::async_trait]
pub trait Persistence {
    async fn get_chart_dto(
        &self,
        connection: &sqlx::Pool<Postgres>,
        data: &Envelope
    ) -> Result<Rc<dyn API>, String>;
    
    async fn transaction_get_chart_dto(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        data: &Envelope
    ) -> Result<Rc<dyn API>, String>;
}

pub trait ChartGenerator: Persistence {
    fn generate_chart(&self, transaction: &mut Transaction<Postgres>, data: &Envelope)
    -> Result<Rc<dyn API>, String>
    {
        block_on(self.transaction_get_chart_dto(transaction, data))
    }
    fn get_requesting_type(&self) -> &'static str;
}

pub mod bandwidth_per_endpoint;
pub mod network_bandwidth;
pub mod network_graph;