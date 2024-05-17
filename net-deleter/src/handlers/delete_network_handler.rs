use std::error::Error;
use std::sync::Arc;

use net_deleter_api::api::network::DeleteNetworkRequestDTO;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_primitives::api::integer::Integer;
use sqlx::Pool;
use sqlx::Postgres;

use crate::core::delete_error::DeleteError;
use crate::utils::network_deleter;

#[derive(Default, Debug)]
pub struct DeleteNetworkHandler {}

impl DeleteNetworkHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for DeleteNetworkHandler {
    // need to trigger refreshes
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        let deletable_data_type = self.get_handler_type().split('-').collect::<Vec<_>>().join(" ");
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(DeleteError::WrongDeletableData(
                deletable_data_type
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let network_to_delete = DeleteNetworkRequestDTO::decode(enveloped_request.get_data());
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(DeleteError::TranscationError(err.to_string()).into()),
        };
        let delete_packets_res = network_deleter::delete_network_transaction(
            &mut transaction,
            network_to_delete.get_id(),
            tenant_id,
        ).await;
        match delete_packets_res {
            Ok(updated_rows) => {
                let _ = transaction.commit().await;
                Ok(Envelope::new(tenant_id, Integer::get_data_type(), &Integer::new(updated_rows.rows_affected() as i64).encode()))
            },
            Err(err) => Err(DeleteError::DbError(deletable_data_type, err).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        DeleteNetworkRequestDTO::get_data_type().to_string()
    }
}
