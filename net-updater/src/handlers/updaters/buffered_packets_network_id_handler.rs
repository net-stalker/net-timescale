use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::primitives::none::None;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_updater_api::api::updaters::update_packets_network_id::update_buffered_packets_network_id_request::UpdateBufferedPacketsNetworkIdRequestDTO;
use sqlx::Pool;
use sqlx::Postgres;
use crate::core::update_error::UpdateError;
use crate::utils::buffered_packets_network_id_updater;

#[derive(Default, Debug)]
pub struct UpdateBufferedPacketsNetworkIdHandler {}

impl UpdateBufferedPacketsNetworkIdHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait]
impl NetworkServiceHandler for UpdateBufferedPacketsNetworkIdHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(UpdateError::WrongUpdatableData(
                self.get_handler_type()
                .split('-')
                .collect::<Vec<_>>()
                .join(" ")
            ).into());
        }
        let tenant_id = enveloped_request.get_tenant_id();
        let update_packets_network_id_request = UpdateBufferedPacketsNetworkIdRequestDTO::decode(enveloped_request.get_data());
        let mut transaction = match connection_pool.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(UpdateError::TranscationError(err.to_string()).into()),
        };
        let network_id = update_packets_network_id_request.get_network_id();
        let packets_ids = update_packets_network_id_request.get_packets_ids().iter().map(|id| id.as_str()).collect::<Vec<&str>>();
        let update_result = buffered_packets_network_id_updater::update_buffered_packets_network_id_transaction(
            &mut transaction,
            network_id,
            &packets_ids,
            tenant_id,
        ).await;
        match update_result {
            Ok(_) => {
                let _ = transaction.commit().await;
                Ok(Envelope::new(tenant_id, None::get_data_type(), &None::default().encode()))
            },
            Err(e) => Err(UpdateError::DbError(self.get_handler_type(), e).into()),
        }
    }

    fn get_handler_type(&self) -> String {
        UpdateBufferedPacketsNetworkIdRequestDTO::get_data_type().to_string()
    }
}
