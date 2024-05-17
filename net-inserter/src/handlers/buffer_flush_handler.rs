use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::typed_api::Typed;
use net_inserter_api::api::buffer::FlushBufferRequestDTO;
use sqlx::{Pool, Postgres};
use crate::core::insert_error::InsertError;
use crate::utils::buffer_flusher;

#[derive(Default, Debug)]
pub struct FlushBufferHandler {}

impl FlushBufferHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait]
impl NetworkServiceHandler for FlushBufferHandler {
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
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(InsertError::TranscationError(err.to_string()).into()),
        };
        let flush_result = buffer_flusher::flush_buffer_transaction(
            &mut transaction,
            tenant_id,
        ).await;
        match flush_result {
            Ok(_) => {
                let _ = transaction.commit().await;
                // TODO: probably we need to return something more than nothing
                Ok(Envelope::new(tenant_id, "none", b""))
            },
            Err(e) => Err(InsertError::DbError(self.get_handler_type(), e).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        FlushBufferRequestDTO::get_data_type().to_string()
    }
}
