use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_reporter_api::api::buffer::buffer_request::BufferRequestDTO;
use net_reporter_api::api::network_packet::network_packets::NetworkPacketsDTO;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use sqlx::Transaction;

use crate::handlers::network_packet_handlers::network_packets::response::network_packet::NetworkPacket;
use crate::handlers::network_packet_handlers::network_packets::response::network_packets::NetworkPackets;


const GET_BUFFER_QUERY: &str = "
    SELECT
        Traffic_Buffer.Pcap_ID AS id,
        Traffic_Buffer.Network_Id AS network_id,
        Traffic_Buffer.Insertion_Time AS insertion_time,
        Traffic_Buffer.Parsed_Data->'l3'->'ip'->>'ip.src' AS src,
        Traffic_Buffer.Parsed_Data->'l3'->'ip'->>'ip.dst' AS dst,
        string_to_array(Traffic_Buffer.Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') AS protocols,
        Traffic_Buffer.Parsed_Data As json_data
    FROM Traffic_Buffer
    WHERE
        Parsed_Data->'l3'->'ip'->>'ip.src' is not null
        AND Parsed_Data->'l3'->'ip'->>'ip.dst' is not null
        AND Traffic_Buffer.Tenant_Id = $1
    GROUP BY Traffic_Buffer.Pcap_ID
    UNION
    SELECT
        Traffic_Buffer.Pcap_ID AS id,
        Traffic_Buffer.Network_Id AS network_id,
        Traffic_Buffer.Insertion_Time AS insertion_time,
        Traffic_Buffer.Parsed_Data->'l3'->'ipv6'->>'ipv6.src' AS src,
        Traffic_Buffer.Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' AS dst,
        string_to_array(Traffic_Buffer.Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') AS protocols,
        Traffic_Buffer.Parsed_Data As json_data
    FROM Traffic_Buffer
    WHERE
        Parsed_Data->'l3'->'ipv6'->>'ipv6.src' is not null
        AND Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' is not null
        AND Traffic_Buffer.Tenant_Id = $1
    GROUP BY Traffic_Buffer.Pcap_ID;
";

#[derive(Default)]
pub struct BufferHandler {}

impl BufferHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        transcation: &mut Transaction<'_, Postgres>,
        tenant_id: &str,
    ) -> Result<Vec<NetworkPacket>, Error> {
        sqlx::query_as(GET_BUFFER_QUERY)
            .bind(tenant_id)
            .fetch_all(&mut **transcation)
            .await
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for BufferHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let mut transcaction = connection_pool.begin().await?;

        let executed_query_response = Self::execute_query(
            &mut transcaction,
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
        BufferRequestDTO::get_data_type().to_string()
    }
}