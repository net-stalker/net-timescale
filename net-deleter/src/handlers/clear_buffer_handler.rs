use std::error::Error;
use std::sync::Arc;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::encoder_api::Encoder;
use net_deleter_api::api::buffer::ClearBufferRequestDTO;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::typed_api::Typed;
use sqlx::Pool;
use sqlx::Postgres;

use crate::core::delete_error::DeleteError;
use crate::utils::buffer_cleaner;

#[derive(Default, Debug)]
pub struct ClearBufferHandler {}

impl ClearBufferHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for ClearBufferHandler {
    // doesn't need to trigger refreshes
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        let deletable_data_type = self.get_handler_type().split('-').collect::<Vec<_>>().join(" ");
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(DeleteError::WrongDeletableData(
                deletable_data_type
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(DeleteError::TranscationError(err.to_string()).into()),
        };
        let delete_packets_res = buffer_cleaner::clear_buffer_transaction(
            &mut transaction,
            tenant_id,
        ).await;
        if let Err(err) = delete_packets_res {
            return Err(DeleteError::DbError(deletable_data_type, err).into());
        }
        let _ = transaction.commit().await;
        Ok(Envelope::new(tenant_id, ResultDTO::get_data_type(), &ResultDTO::new(true, None, None).encode()))
    }

    fn get_handler_type(&self) -> String {
        ClearBufferRequestDTO::get_data_type().to_string()
    }
}
