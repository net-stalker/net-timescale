use std::sync::Arc;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_filters::NetworkBandwidthPerEndpointFiltersDTO;
use net_token_verifier::fusion_auth::jwt_token::Jwt;
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


const EXCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND not (ARRAY(SELECT DISTINCT unnest(string_to_array(protocols, ':'))) && {})
";

const INCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND ARRAY(SELECT DISTINCT unnest(string_to_array(protocols, ':'))) @> {}
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (COALESCE(lhs.id, rhs.id) IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (COALESCE(lhs.id, rhs.id) NOT IN (SELECT unnest({})))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND LEAST(COALESCE(lhs.bytes_sent, 0), COALESCE(rhs.bytes_received, 0)) >= {}
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND GREATEST(COALESCE(lhs.bytes_sent, 0), COALESCE(rhs.bytes_received, 0)) < {}
";

const NETWORK_BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY: &str = "
    SELECT
        COALESCE(lhs.id, rhs.id) AS id,
        COALESCE(lhs.bytes_sent, 0) AS bytes_sent,
        COALESCE(rhs.bytes_received, 0) AS bytes_received
    FROM
    (
        SELECT
            src_addr AS id,
            SUM(packet_length) AS bytes_sent
        FROM bandwidth_per_endpoint_aggregate
        WHERE 
            group_id = $1
            AND bucket >= $2
            AND bucket < $3
            {}
        GROUP BY src_addr
    ) AS lhs full outer join (
        SELECT
            dst_addr AS id,
            SUM(packet_length) AS bytes_received
        FROM bandwidth_per_endpoint_aggregate
        WHERE 
            group_id = $1
            AND bucket >= $2 
            AND bucket < $3
            {}
        GROUP BY dst_addr
    ) AS rhs ON lhs.id = rhs.id
    WHERE
        1 = 1
        {}
        {}
        {}
    ORDER BY id;
";

#[derive(Default)]
pub struct NetworkBandwidthPerEndpointRequester {}

impl NetworkBandwidthPerEndpointRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn get_query_based_on_requested_filters(filters: &NetworkBandwidthPerEndpointFiltersDTO) -> String {
        let mut placeholder_value = 4;
        let mut request_query = NETWORK_BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY.to_owned();

        match filters.is_include_protocols_mode() {
            Some(true) => {
                let protocols_query = INCLUDE_PROTOCOLS_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", protocols_query.as_str(), 2);
            },
            Some(false) => {
                let protocols_query = EXCLUDE_PROTOCOLS_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", protocols_query.as_str(), 2);
            },
            None => request_query = request_query.replacen("{}", "", 2)
        }

        match filters.is_include_endpoints_mode() {
            Some(true) => {
                let endpoints_query = INCLUDE_ENDPOINT_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", endpoints_query.as_str(), 1);
            },
            Some(false) => { 
                let endpoints_query = EXCLUDE_ENDPOINT_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", endpoints_query.as_str(), 1) 
            },
            None => request_query = request_query.replacen("{}", "", 1)
        };

        match filters.get_bytes_lower_bound() {
            Some(_) => {
                let lower_bytes_query = SET_LOWER_BYTES_BOUND.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", lower_bytes_query.as_str(), 1)
            },
            None => request_query = request_query.replacen("{}", "", 1)
        };

        match filters.get_bytes_upper_bound() {
            Some(_) => {
                let upper_bytes_query = SET_UPPER_BYTES_BOUND.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                request_query = request_query.replacen("{}", upper_bytes_query.as_str(), 1)
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
        filters: &NetworkBandwidthPerEndpointFiltersDTO,
    ) -> Result<Vec<EndpointResponse>, Error> {
        let request_query = Self::get_query_based_on_requested_filters(filters).await;

        log::debug!("Request query: {}", request_query);
        log::debug!("Group id: {:?}", group_id);
        log::debug!("Start date: {}", start_date);
        log::debug!("End date: {}", end_date);
        log::debug!("Filters: {:?}", filters);


        let mut sqlx_query = sqlx::query_as(request_query.as_str())
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

        sqlx_query = match filters.get_bytes_lower_bound() {
            Some(lower_bound) => sqlx_query.bind(lower_bound),
            None => sqlx_query,
        };

        sqlx_query = match filters.get_bytes_upper_bound() {
            Some(upper_bound) => sqlx_query.bind(upper_bound),
            None => sqlx_query,
        };

        sqlx_query.fetch_all(connection_pool.as_ref())
            .await
    }
}

#[async_trait::async_trait]
impl Requester for NetworkBandwidthPerEndpointRequester {
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
        let request = NetworkBandwidthPerEndpointRequestDTO::decode(enveloped_request.get_data());
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

        let response: NetworkBandwidthPerEndpointResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthPerEndpointDTO = response.into();

        Ok(Envelope::new(
            enveloped_request.get_jwt_token().ok(),
            request_agent_id,
            NetworkBandwidthPerEndpointDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthPerEndpointRequestDTO::get_data_type()
    }
}