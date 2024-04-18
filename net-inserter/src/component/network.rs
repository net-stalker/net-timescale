use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::api::API;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;
use net_inserter_api::api::network::InsertNetworkRequestDTO;
use sqlx::Postgres;
use crate::core::insert_error::InsertError;
use crate::core::insert_handler::InsertHandler;
use crate::utils::network_inserter;

#[derive(Default, Debug)]
pub struct InsertNetworkHandler {}

impl InsertNetworkHandler {}

#[async_trait]
impl InsertHandler for InsertNetworkHandler {
    async fn insert(&self, transaction: &mut sqlx::Transaction<'_, Postgres>, data_to_insert: Envelope) -> Result<Option<Arc<dyn API + Send + Sync>>, Box<dyn Error + Send + Sync>> {
        if data_to_insert.get_envelope_type() != self.get_insertable_data_type() {
            return Err(Box::new(InsertError::WrongInsertableData(
                self.get_insertable_data_type()
                .split('-')
                .collect::<Vec<_>>()
                .join(" ")
            )))
        }
        let tenant_id = data_to_insert.get_tenant_id();
        let network_data = InsertNetworkRequestDTO::decode(data_to_insert.get_data());
        let insert_result = network_inserter::insert_network_transaction(
            transaction,
            tenant_id,
            &network_data,
        ).await;
        match insert_result {
            Ok(_) => Ok(None),
            Err(e) => Err(Box::new(InsertError::DbError(self.get_insertable_data_type().to_string(), e))),
        }
    }

    fn get_insertable_data_type(&self) -> &'static str {
        InsertNetworkRequestDTO::get_data_type()
    }
}
