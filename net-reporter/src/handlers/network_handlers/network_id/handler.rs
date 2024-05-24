use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_reporter_api::api::network::network_id::NetworkIdDTO;
use net_reporter_api::api::network::network_id_request::NetworkIdRequestDTO;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use sqlx::Transaction;

use super::response::network_id::NetworkId;

const GET_NETWORK_ID_QUERY: &str = "
    SELECT Networks.Network_ID AS id
    FROM Networks
    WHERE Networks.Network_Name = $1 AND Networks.Tenant_ID = $2;
";

#[derive(Default)]
pub struct NetworkIdHandler {}

impl NetworkIdHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        transcation: &mut Transaction<'_, Postgres>,
        network_name: &str,
        tenant_id: &str,
    ) -> Result<NetworkId, Error> {
        sqlx::query_as(GET_NETWORK_ID_QUERY)
            .bind(network_name)
            .bind(tenant_id)
            .fetch_one(&mut **transcation)
            .await
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for NetworkIdHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworkIdRequestDTO::decode(enveloped_request.get_data());
        let network_name = request.get_name();
        let mut transcaction = connection_pool.begin().await?;

        let executed_query_response = Self::execute_query(
            &mut transcaction,
            network_name,
            tenant_id,
        ).await?;

        transcaction.commit().await?;
        log::debug!("Got response on request: {:?}", executed_query_response);

        let dto_response: NetworkIdDTO = executed_query_response.into();

        Ok(Envelope::new(
            tenant_id,
            dto_response.get_type(),
            &dto_response.encode(),
        ))
    }
    
    fn get_handler_type(&self) -> String {
        NetworkIdRequestDTO::get_data_type().to_string()
    }
}