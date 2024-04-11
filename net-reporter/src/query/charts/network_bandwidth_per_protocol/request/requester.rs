use std::sync::Arc;

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

use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolDTO;
use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_request::NetworkBandwidthPerProtocolRequestDTO;
use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_filters::NetworkBandwidthPerProtocolFiltersDTO;

use crate::query::charts::network_bandwidth_per_protocol::response::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolResponse;
use crate::query::charts::network_bandwidth_per_protocol::response::protocol::ProtocolResponse;
use crate::query::requester::Requester;
use crate::query_builder::query_builder::QueryBuilder;
use crate::query_builder::sqlx_query_builder_wrapper::SqlxQueryBuilderWrapper;

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
    GROUP BY separated_protocols
    HAVING
        1 = 1
        {}
        {};
";

#[derive(Default)]
pub struct NetworkBandwidthPerProtocolRequester {}

impl NetworkBandwidthPerProtocolRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        query_string: &str,
        group_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &NetworkBandwidthPerProtocolFiltersDTO,
    ) -> Result<Vec<ProtocolResponse>, Error> {
        SqlxQueryBuilderWrapper::<ProtocolResponse>::new(query_string)
            .add_param(group_id)
            .add_param(start_date)
            .add_param(end_date)
            .add_option_param(filters.is_include_endpoints_mode().map(|_| filters.get_endpoints().to_vec()))
            .add_option_param(filters.get_bytes_lower_bound())
            .add_option_param(filters.get_bytes_upper_bound())
            .execute_query(connection_pool).await
    }
}

#[async_trait::async_trait]
impl Requester for NetworkBandwidthPerProtocolRequester {
    async fn request_enveloped_chart(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let group_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworkBandwidthPerProtocolRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let request_filters = request.get_filters();

        let query = QueryBuilder::new(NETWORK_BANDWIDTH_PER_PROTOCOL_REQUEST_QUERY, 4)
            .add_dynamic_filter(request_filters.is_include_endpoints_mode(), 1, INCLUDE_ENDPOINT_FILTER_QUERY, EXCLUDE_ENDPOINT_FILTER_QUERY)
            .add_static_filter(request_filters.get_bytes_lower_bound(), SET_LOWER_BYTES_BOUND, 1)
            .add_static_filter(request_filters.get_bytes_upper_bound(), SET_UPPER_BYTES_BOUND, 1)
            .build_query();

        let executed_query_response = Self::execute_query(
            connection_pool,
            query.as_str(),
            group_id,
            request_start_date,
            request_end_date,
            request_filters,
        ).await?;

        let response: NetworkBandwidthPerProtocolResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthPerProtocolDTO = response.into();

        Ok(Envelope::new(
            group_id,
            NetworkBandwidthPerProtocolDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthPerProtocolRequestDTO::get_data_type()
    }
}