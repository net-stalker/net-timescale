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
use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolDTO;
use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_request::NetworkBandwidthPerProtocolRequestDTO;
use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_filters::NetworkBandwidthPerProtocolFiltersDTO;

use crate::query::charts::network_bandwidth_per_protocol::response::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolResponse;
use crate::query::charts::network_bandwidth_per_protocol::response::protocol::ProtocolResponse;
use crate::query::requester::Requester;

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr IN (SELECT unnest({})) OR dst_addr IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr NOT IN (SELECT unnest({})) AND dst_addr NOT IN (SELECT unnest({})))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND SUM(packet_length) >= {}
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND SUM(packet_length) < {}
";

const NETWORK_BANDWIDTH_PER_PROTOCOL_REQUEST_QUERY: &str = "
    SELECT SUM(packet_length) AS total_bytes, separated_protocols AS protocol_name
    FROM (
        SELECT *, UNNEST(STRING_TO_ARRAY(protocols, ':')) AS separated_protocols
        FROM bandwidth_per_protocol_aggregate
    ) AS unnested_protocols
    WHERE
        group_id = $1
        AND bucket >= $2
        AND bucket < $3
        {}
        {}
        {}
    GROUP BY separated_protocols;
";

#[derive(Default)]
pub struct NetworkBandwidthPerProtocolRequester {}

impl NetworkBandwidthPerProtocolRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn get_query_based_on_requested_filters(filters: &NetworkBandwidthPerProtocolFiltersDTO) -> String {
        let mut placeholder_value = 4;
        let mut request_query = NETWORK_BANDWIDTH_PER_PROTOCOL_REQUEST_QUERY.to_owned();

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
        filters: &NetworkBandwidthPerProtocolFiltersDTO,
    ) -> Result<Vec<ProtocolResponse>, Error> {
        let request_query = Self::get_query_based_on_requested_filters(filters).await;

        let mut sqlx_query = sqlx::query_as(request_query.as_str())
            .bind(group_id)
            .bind(start_date)
            .bind(end_date);
        
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

        sqlx_query.fetch_all(connection_pool.as_ref()).await
    }
}

#[async_trait::async_trait]
impl Requester for NetworkBandwidthPerProtocolRequester {
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
        let request = NetworkBandwidthPerProtocolRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let request_filters = request.get_filters();

        let executed_query_response = Self::execute_query(
            connection_pool,
            request_group_id,
            request_start_date,
            request_end_date,
            request_filters,
        ).await;

        if let Err(e) = executed_query_response {
            return Err(format!("error: {:?}", e));
        }
        let executed_query_response = executed_query_response.unwrap();

        let response: NetworkBandwidthPerProtocolResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthPerProtocolDTO = response.into();

        Ok(Envelope::new(
            request_group_id,
            request_agent_id,
            NetworkBandwidthPerEndpointDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthPerProtocolRequestDTO::get_data_type()
    }
}