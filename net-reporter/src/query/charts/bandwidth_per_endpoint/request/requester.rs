use std::sync::Arc;

use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;
use net_core_api::decoder_api::Decoder;
use net_core_api::encoder_api::Encoder;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;
use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO;

use crate::query::charts::bandwidth_per_endpoint::response::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointResponse;
use crate::query::charts::bandwidth_per_endpoint::response::endpoint::EndpointResponse;
use crate::query::requester::Requester;

const NETWORK_BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY: &str = "
    SELECT
    COALESCE(lhs.id, rhs.id) AS id,
    COALESCE(lhs.bytes_sent, 0) AS bytes_sent,
    COALESCE(rhs.bytes_received, 0) AS bytes_received,
    ARRAY(select distinct unnest(array_cat(lhs.protocols, rhs.protocols))) as protocols
    FROM
    (
        SELECT
            src_addr AS id,
            SUM(packet_length) AS bytes_sent,
            ARRAY(SELECT DISTINCT unnest(string_to_array(string_agg(protocols, ':'), ':'))) AS protocols
        FROM bandwidth_per_endpoint_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
        GROUP BY src_addr
    ) AS lhs full outer join (
        SELECT
            dst_addr AS id,
            SUM(packet_length) AS bytes_received,
            ARRAY(SELECT DISTINCT unnest(string_to_array(string_agg(protocols, ':'), ':'))) AS protocols
        FROM bandwidth_per_endpoint_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
        GROUP BY dst_addr
    ) AS rhs ON lhs.id = rhs.id;
";

#[derive(Default)]
pub struct NetworkBandwidthPerEndpointRequester {}

impl NetworkBandwidthPerEndpointRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<EndpointResponse>, Error> {
        sqlx::query_as(NETWORK_BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await
    }
}

#[async_trait::async_trait]
impl Requester for NetworkBandwidthPerEndpointRequester {
    async fn request(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope
    ) -> Result<Envelope, String> {
        let request_group_id = enveloped_request.get_group_id().ok();
        let request_agent_id = enveloped_request.get_agent_id().ok();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()));
        }
        let request = NetworkBandwidthPerEndpointRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();

        let executed_query_response = Self::execute_query(
            connection_pool,
            request_group_id,
            request_start_date,
            request_end_date
        ).await;

        if let Err(e) = executed_query_response {
            return Err(format!("error: {:?}", e));
        }
        let executed_query_response = executed_query_response.unwrap();

        let response: NetworkBandwidthPerEndpointResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthPerEndpointDTO = response.into();

        Ok(Envelope::new(
            request_group_id,
            request_agent_id,
            NetworkBandwidthPerEndpointDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthPerEndpointRequestDTO::get_data_type()
    }
}