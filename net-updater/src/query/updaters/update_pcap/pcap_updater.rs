use std::error::Error;
use std::sync::Arc;

use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;

use net_updater_api::api::updaters::update_pcap::update_pcap_request::UpdatePcapRequestDTO;

use crate::core::request_result::RequestResult;
use crate::core::update_handler::UpdateHandler;

const NETWORK_UPDATE_QUERY: &str = "
UPDATE Traffic SET Network_ID = $1
WHERE 
    Pcap_ID = $2;
";

pub struct PcapUpdater {}

impl PcapUpdater {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        query_string: &str,
        network_id: Option<i64>,
        pcap_id: i64,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(query_string)
            .bind(network_id)
            .bind(pcap_id)
            .execute(connection_pool.as_ref())
            .await
    }
}

#[async_trait::async_trait]
impl UpdateHandler for PcapUpdater {
    async fn update(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        if enveloped_request.get_envelope_type() != self.get_updating_request_type() {
            todo!()
        }

        let update_pcap_request = UpdatePcapRequestDTO::decode(enveloped_request.get_data());

        PcapUpdater::execute_query(
            connection_pool,
            NETWORK_UPDATE_QUERY,
            update_pcap_request.get_network_id(),
            update_pcap_request.get_id(),
        ).await?;

        let result_to_return: ResultDTO = RequestResult::ok(None, None).into();

        Ok(Envelope::new(
            enveloped_request.get_tenant_id(),
            ResultDTO::get_data_type(),
            &result_to_return.encode()
        ))
    }
    
    fn get_updating_request_type(&self) -> &'static str {
        UpdatePcapRequestDTO::get_data_type()
    }
}