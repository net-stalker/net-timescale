use std::rc::Rc;
use net_proto_api::api::API;
use net_proto_api::envelope::envelope::Envelope;
use sqlx::{Postgres, Transaction};
use async_std::task::block_on;
pub mod network_graph;

#[async_trait::async_trait]
pub trait Persistence {
    type DTO: API + 'static;
    async fn get_dto(
        &self,
        connection: &sqlx::Pool<Postgres>,
        data: &Envelope
    ) -> Result<Self::DTO, String>;

    async fn transaction_get_dto(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        data: &Envelope
    ) -> Result<Self::DTO, String>;
}

pub trait ChartGenerator: Persistence {
    fn generate_chart(&self, transaction: &mut Transaction<Postgres>, data: &Envelope)
                      -> Result<Rc<dyn API>, String>
    {
        match block_on(self.transaction_get_dto(transaction, data)) {
            Ok(dto) => Ok(Rc::new(dto)),
            Err(err) => Err(err)
        }
    }
    fn get_requesting_type(&self) -> &'static str;
}