use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_reporter_api::api::network::networks::NetworksDTO;
use net_reporter_api::api::network::networks_request::NetworksRequestDTO;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use sqlx::Transaction;

use crate::handlers::network_handlers::networks::response::networks::Networks;

use super::response::network::Network;

const GET_NETWORKS_QUERY: &str = "
    SELECT Networks.Network_ID AS id, Networks.Network_Name AS name, Networks.Network_Color AS color
    FROM Networks
    WHERE
        (COALESCE(ARRAY_LENGTH($1, 1), 0) = 0 OR Networks.Network_ID IN (SELECT UNNEST($1)))
        AND Networks.Tenant_Id = $2;
";

#[derive(Default)]
pub struct NetworksHandler {}

impl NetworksHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        transcation: &mut Transaction<'_, Postgres>,
        network_ids: &[String],
        tenant_id: &str,
    ) -> Result<Vec<Network>, Error> {
        sqlx::query_as(GET_NETWORKS_QUERY)
            .bind(network_ids)
            .bind(tenant_id)
            .fetch_all(&mut **transcation)
            .await
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for NetworksHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworksRequestDTO::decode(enveloped_request.get_data());
        let network_ids = request.get_networks_ids_to_get();
        let mut transcaction = connection_pool.begin().await?;

        let executed_query_response = Self::execute_query(
            &mut transcaction,
            network_ids,
            tenant_id,
        ).await?;
        transcaction.commit().await?;
        let response: Networks = executed_query_response.into(); 
        log::debug!("Got response on request: {:?}", response);

        let dto_response: NetworksDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            dto_response.get_type(),
            &dto_response.encode(),
        ))
    }
    
    fn get_handler_type(&self) -> String {
        NetworksRequestDTO::get_data_type().to_string()
    }
}