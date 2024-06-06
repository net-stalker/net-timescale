use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::primitives::none::None;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::encoder_api::Encoder;
use net_inserter_api::api::network::InsertNetworkRequestDTO;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::core::insert_error::InsertError;
use crate::utils::network_inserter;

#[derive(Default, Debug)]
pub struct InsertNetworkHandler {}

impl InsertNetworkHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait]
impl NetworkServiceHandler for InsertNetworkHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(InsertError::WrongInsertableData(
                self.get_handler_type()
                .split('-')
                .collect::<Vec<_>>()
                .join(" ")
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let network_data = InsertNetworkRequestDTO::decode(enveloped_request.get_data());
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(InsertError::TranscationError(err.to_string()).into()),
        };
        let network_id = Uuid::now_v7().to_string();
        let insert_result = network_inserter::insert_network_transaction(
            &mut transaction,
            tenant_id,
            &network_id,
            &network_data,
        ).await;
        match insert_result {
            Ok(_) => {
                let _ = transaction.commit().await;
                Ok(Envelope::new(tenant_id, None::get_data_type(), &None::default().encode()))
            },
            Err(e) => Err(InsertError::DbError(self.get_handler_type(), e).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        InsertNetworkRequestDTO::get_data_type().to_string()
    }
}
