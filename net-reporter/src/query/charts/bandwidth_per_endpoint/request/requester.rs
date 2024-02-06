use std::sync::Arc;

use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::typed_api::Typed;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;

use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint::BandwidthPerEndpointDTO;
use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint_request::BandwidthPerEndpointRequestDTO;

use crate::query::charts::bandwidth_per_endpoint::response::bandwidth_per_endpoint::BandwidthPerEndpointResponse;
use crate::query::charts::bandwidth_per_endpoint::response::endpoint::EndpointResponse;
use crate::query::requester::Requester;

const BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY: &str = "
    SELECT
    COALESCE(lhs.id, rhs.id) AS id,
    lhs.bytes_sent AS bytes_sent,
    rhs.bytes_received AS bytes_received
    FROM
    (
        SELECT
            src_addr AS id,
            SUM(packet_length) AS bytes_sent
        FROM bandwidth_per_endpoint_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
        GROUP BY src_addr
    ) AS lhs full outer join (
        SELECT
            dst_addr AS id,
            SUM(packet_length) AS bytes_received
        FROM bandwidth_per_endpoint_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
        GROUP BY dst_addr
    ) AS rhs ON lhs.id = rhs.id;
";

#[derive(Default)]
pub struct BandwidthPerEndpointRequester {}

impl BandwidthPerEndpointRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<EndpointResponse>, Error> {
        sqlx::query_as(BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await
    }
}

#[async_trait::async_trait]
impl Requester for BandwidthPerEndpointRequester {
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
        let request = BandwidthPerEndpointRequestDTO::decode(enveloped_request.get_data());
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

        let response: BandwidthPerEndpointResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: BandwidthPerEndpointDTO = response.into();

        Ok(Envelope::new(
            request_group_id,
            request_agent_id,
            BandwidthPerEndpointDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        BandwidthPerEndpointRequestDTO::get_data_type()
    }
}