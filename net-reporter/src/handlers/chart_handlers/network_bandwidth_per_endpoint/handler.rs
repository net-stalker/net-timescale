use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;

use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;
use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_filters::NetworkBandwidthPerEndpointFiltersDTO;
use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO;

use crate::handlers::chart_handlers::network_bandwidth_per_endpoint::response::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointResponse;
use crate::handlers::chart_handlers::network_bandwidth_per_endpoint::response::endpoint::EndpointResponse;
use crate::query_builder::query_builder::QueryBuilder;
use crate::query_builder::sqlx_query_builder_wrapper::SqlxQueryBuilderWrapper;


const EXCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND NOT (ARRAY(SELECT DISTINCT unnest(string_to_array(Protocols, ':'))) && {})
";

const INCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND ARRAY(SELECT DISTINCT unnest(string_to_array(Protocols, ':'))) @> {}
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (COALESCE(lhs.IP, rhs.IP) IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (COALESCE(lhs.IP, rhs.IP) NOT IN (SELECT unnest({})))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND LEAST(COALESCE(lhs.Bytes_Sent, 0), COALESCE(rhs.Bytes_Received, 0)) >= {}
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND GREATEST(COALESCE(lhs.Bytes_Sent, 0), COALESCE(rhs.Bytes_Received, 0)) < {}
";

const NETWORK_BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY: &str = "
    SELECT
        COALESCE(lhs.IP, rhs.IP) AS IP,
        COALESCE(lhs.Bytes_Sent, 0) AS Bytes_Sent,
        COALESCE(rhs.Bytes_Received, 0) AS Bytes_Received
    FROM
    (
        SELECT
            Src_IP AS IP,
            SUM(Packet_Length) AS Bytes_Sent
        FROM Network_Bandwidth_Per_Endpoint_Materialized_View
        WHERE 
            Tenant_ID = $1
            AND Frametime >= $2
            AND Frametime < $3
            AND Network_ID = $4
            {}
        GROUP BY Src_IP
    ) AS lhs FULL OUTER JOIN (
        SELECT
            Dst_IP AS IP,
            SUM(Packet_Length) AS Bytes_Received
        FROM Network_Bandwidth_Per_Endpoint_Materialized_View
        WHERE 
            Tenant_ID = $1
            AND Frametime >= $2 
            AND Frametime < $3
            AND Network_ID = $4
            {}
        GROUP BY Dst_IP
    ) AS rhs ON lhs.IP = rhs.IP
    WHERE
        1 = 1
        {}
        {}
        {}
    ORDER BY IP;
";

#[derive(Default)]
pub struct NetworkBandwidthPerEndpointHandler {}

impl NetworkBandwidthPerEndpointHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        query_string: &str,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        network_id: &str,
        filters: &NetworkBandwidthPerEndpointFiltersDTO,
    ) -> Result<Vec<EndpointResponse>, Error> {
        SqlxQueryBuilderWrapper::<EndpointResponse>::new(query_string)
            .add_param(tenant_id)
            .add_param(start_date)
            .add_param(end_date)
            .add_param(network_id)
            .add_option_param(filters.is_include_protocols_mode().map(|_| filters.get_protocols().to_vec()))
            .add_option_param(filters.is_include_endpoints_mode().map(|_| filters.get_endpoints().to_vec()))
            .add_option_param(filters.get_bytes_lower_bound())
            .add_option_param(filters.get_bytes_upper_bound())
            .execute_query(connection_pool).await
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for NetworkBandwidthPerEndpointHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_envelope_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworkBandwidthPerEndpointRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let network_id = request.get_network_id();
        let filters = request.get_filters();

        let query = QueryBuilder::new(NETWORK_BANDWIDTH_PER_ENDPOINT_REQUEST_QUERY, 5)
            .add_dynamic_filter(filters.is_include_protocols_mode(), 2, INCLUDE_PROTOCOLS_FILTER_QUERY, EXCLUDE_PROTOCOLS_FILTER_QUERY)
            .add_dynamic_filter(filters.is_include_endpoints_mode(), 1, INCLUDE_ENDPOINT_FILTER_QUERY, EXCLUDE_ENDPOINT_FILTER_QUERY)
            .add_static_filter(filters.get_bytes_lower_bound(), SET_LOWER_BYTES_BOUND, 1)
            .add_static_filter(filters.get_bytes_upper_bound(), SET_UPPER_BYTES_BOUND, 1)
            .build_query();

        let executed_query_response = Self::execute_query(
            connection_pool,
            query.as_str(),
            tenant_id,
            request_start_date,
            request_end_date,
            network_id,
            filters,
        ).await?;

        let response: NetworkBandwidthPerEndpointResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthPerEndpointDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            NetworkBandwidthPerEndpointDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_handler_type(&self) -> String {
        NetworkBandwidthPerEndpointRequestDTO::get_data_type().to_string()
    }
}