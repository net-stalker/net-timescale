use std::error::Error;
use std::sync::Arc;

use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_updater_api::api::updaters::update_network::update_network_request::UpdateNetworkRequestDTO;
use sqlx::postgres::PgQueryResult;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;

use crate::core::request_result::RequestResult;
use crate::core::update_handler::UpdateHandler;

const NETWORK_UPDATE_QUERY: &str = "
UPDATE Networks SET Network_Name = $1, Network_Color = $2
WHERE 
    Network_ID = $3;
";

pub struct NetworkUpdater {}

impl NetworkUpdater {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        query_string: &str,
        network_name: &str,
        network_color: &str,
        network_id: i64,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(query_string)
            .bind(network_name)
            .bind(network_color)
            .bind(network_id)
            .execute(connection_pool.as_ref())
            .await
    }
}

#[async_trait::async_trait]
impl UpdateHandler for NetworkUpdater {
    async fn update(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        if enveloped_request.get_envelope_type() != self.get_updating_request_type() {
            todo!()
        }

        let update_network_request = UpdateNetworkRequestDTO::decode(enveloped_request.get_data());

        NetworkUpdater::execute_query(
            connection_pool,
            NETWORK_UPDATE_QUERY,
            update_network_request.get_name(),
            update_network_request.get_color(),
            update_network_request.get_id()
        ).await?;

        let result_to_return: ResultDTO = RequestResult::ok(None, None).into();

        Ok(Envelope::new(
            enveloped_request.get_tenant_id(),
            ResultDTO::get_data_type(),
            &result_to_return.encode()
        ))
    }
    
    fn get_updating_request_type(&self) -> &'static str {
        UpdateNetworkRequestDTO::get_data_type()
    }
}