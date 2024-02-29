use std::sync::Arc;

use net_token_verifier::fusion_auth::jwt_token::Jwt;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::decoder_api::Decoder;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;

use crate::query::charts::network_bandwidth::response::network_bandwidth::NetworkBandwidthResponse;
use crate::query::charts::network_bandwidth::response::bandwidth_bucket::BandwidthBucketResponse;
use crate::query::requester::Requester;

const EXCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND not (string_to_array(protocols, ':') && {})
";

const INCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND (string_to_array(protocols, ':') @> {})
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr IN (SELECT unnest({})) OR dst_addr IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr NOT IN (SELECT unnest({})) AND dst_addr NOT IN (SELECT unnest({})))
";

const NETWORK_BANDWIDTH_REQUEST_QUERY: &str = "
    SELECT bucket, SUM(packet_length) as total_bytes
    FROM network_bandwidth_aggregate
    WHERE 
        group_id = $1
        AND bucket >= $2
        AND bucket < $3
        {}
        {}
    GROUP BY bucket
    ORDER BY bucket;
";

#[derive(Default)]
pub struct NetworkBandwidthRequester {}

impl NetworkBandwidthRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn get_query_based_on_requested_filters(filters: &NetworkBandwidthFiltersDTO) -> String {
        let mut placeholder_value = 4;
        let mut request_query = NETWORK_BANDWIDTH_REQUEST_QUERY.to_string();

        match filters.is_include_protocols_mode() {
            Some(true) => {
                let protocol_filter_query = INCLUDE_PROTOCOLS_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", protocol_filter_query.as_str(), 1)
            },    
            Some(false) => {
                let protocol_filter_query = EXCLUDE_PROTOCOLS_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", protocol_filter_query.as_str(), 1)
            },
            None => request_query = request_query.replacen("{}", "", 1)
        };

        match filters.is_include_endpoints_mode() {
            Some(true) => {
                let endpoint_filter_query = INCLUDE_ENDPOINT_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                request_query = request_query.replacen("{}", endpoint_filter_query.as_str(), 1)
            },
            Some(false) => {
                let endpoint_filter_query = EXCLUDE_ENDPOINT_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                request_query = request_query.replacen("{}", endpoint_filter_query.as_str(), 1)
            },
            None => request_query = request_query.replacen("{}", "", 1)
        };

        request_query
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &NetworkBandwidthFiltersDTO,
    ) -> Result<Vec<BandwidthBucketResponse>, Error> {
        let request_query = Self::get_query_based_on_requested_filters(filters).await;

        let mut sqlx_query = sqlx::query_as(&request_query)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date);

        sqlx_query = match filters.is_include_protocols_mode() {
            Some(_) => sqlx_query.bind(filters.get_protocols()),
            None => sqlx_query,
        };
        sqlx_query = match filters.is_include_endpoints_mode() {
            Some(_) => sqlx_query.bind(filters.get_endpoints()),
            None => sqlx_query,
        };

        sqlx_query.fetch_all(connection_pool.as_ref()).await
    }
}

#[async_trait::async_trait]
impl Requester for NetworkBandwidthRequester {
    async fn request(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
        jwt: Jwt,
    ) -> Result<Envelope, String> {
        let request_agent_id = enveloped_request.get_agent_id().ok();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()));
        }
        let request = NetworkBandwidthRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let filters = request.get_filters();

        let executed_query_response = Self::execute_query(
            connection_pool,
            Some(jwt.get_tenant_id()),
            request_start_date,
            request_end_date,
            filters,
        ).await;

        if let Err(e) = executed_query_response {
            return Err(format!("error: {:?}", e));
        }
        let executed_query_response = executed_query_response.unwrap();

        let response: NetworkBandwidthResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthDTO = response.into();

        Ok(Envelope::new(
            enveloped_request.get_jwt_token().ok(),
            request_agent_id,
            NetworkBandwidthDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthRequestDTO::get_data_type()
    }
}