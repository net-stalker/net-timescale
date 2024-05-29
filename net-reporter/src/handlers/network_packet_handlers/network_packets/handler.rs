use std::sync::Arc;

use component_core::materialized_view::query;
use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_reporter_api::api::network_packet::network_packets::NetworkPacketsDTO;
use net_reporter_api::api::network_packet::network_packets_request::NetworkPacketsRequestDTO;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use sqlx::Transaction;

use crate::handlers::network_packet_handlers::network_packets::response::network_packets::NetworkPackets;
use crate::query_builder::query_builder::QueryBuilder;

use super::response::network_packet::NetworkPacket;

const GET_NETWORK_PACKETS: &str = "
    SELECT
        Traffic.Pcap_ID AS id,
        Traffic.Network_Id AS network_id,
        Traffic.Insertion_Time AS insertion_time,
        Traffic.Parsed_Data->'l3'->'ip'->>'ip.src' AS src,
        Traffic.Parsed_Data->'l3'->'ip'->>'ip.dst' AS dst,
        string_to_array(Traffic.Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') AS protocols,
        Traffic.Parsed_Data AS json_data
    FROM Traffic
    WHERE
        (
            COALESCE(ARRAY_LENGTH($1, 1), 0) = 0
            OR Traffic.Network_ID = ANY(ARRAY(SELECT UNNEST($1)))
            {}
        )
        AND Traffic.Tenant_Id = $2
    GROUP BY Traffic.Pcap_ID;
";

const SET_NULL_NETWORK: &str = "
    OR Traffic.Network_ID is NULL
";

#[derive(Default)]
pub struct NetworkPacketsHandler {}

impl NetworkPacketsHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        query_string: &str,
        transcation: &mut Transaction<'_, Postgres>,
        network_ids: &[Option<String>],
        tenant_id: &str,
    ) -> Result<Vec<NetworkPacket>, Error> {
        sqlx::query_as(query_string)
            .bind(network_ids)
            .bind(tenant_id)
            .fetch_all(&mut **transcation)
            .await
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for NetworkPacketsHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworkPacketsRequestDTO::decode(enveloped_request.get_data());
        // kekw moment
        let network_ids = request.get_id();
        let mut transcaction = connection_pool.begin().await?;

        let query = QueryBuilder::new(GET_NETWORK_PACKETS, 1)
            .add_static_filter(network_ids.iter().find(|id| **id == None), SET_NULL_NETWORK, 1)
            .build_query();

        let executed_query_response = Self::execute_query(
            &query,
            &mut transcaction,
            network_ids,
            tenant_id,
        ).await?;
        transcaction.commit().await?;
        let response: NetworkPackets = executed_query_response.into(); 
        log::debug!("Got response on request: {:?}", response);

        let dto_response: NetworkPacketsDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            dto_response.get_type(),
            &dto_response.encode(),
        ))
    }
    
    fn get_handler_type(&self) -> String {
        NetworkPacketsRequestDTO::get_data_type().to_string()
    }
}