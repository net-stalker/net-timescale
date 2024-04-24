use std::sync::Arc;

use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;

use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;

use crate::query::charts::network_bandwidth::response::network_bandwidth::NetworkBandwidthResponse;
use crate::query::charts::network_bandwidth::response::bandwidth_bucket::BandwidthBucketResponse;
use crate::query::requester::RequestHandler;
use crate::query_builder::query_builder::QueryBuilder;
use crate::query_builder::sqlx_query_builder_wrapper::SqlxQueryBuilderWrapper;

const EXCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND NOT (string_to_array(Protocols, ':') && {})
";

const INCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND (string_to_array(Protocols, ':') @> {})
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (Src_IP IN (SELECT unnest({})) OR Dst_IP IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (Src_IP NOT IN (SELECT unnest({})) AND Dst_IP NOT IN (SELECT unnest({})))
";

const NETWORK_BANDWIDTH_REQUEST_QUERY: &str = "
    SELECT Frametime, SUM(Packet_Length) AS Total_Bytes
    FROM Network_Bandwidth_Materialized_View
    WHERE 
        Tenant_ID = $1
        AND Frametime >= $2
        AND Frametime < $3
        {}
        {}
    GROUP BY Frametime
    ORDER BY Frametime;
";

#[derive(Default)]
pub struct NetworkBandwidthRequester {}

impl NetworkBandwidthRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        query_string: &str,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &NetworkBandwidthFiltersDTO,
    ) -> Result<Vec<BandwidthBucketResponse>, Error> {
        SqlxQueryBuilderWrapper::<BandwidthBucketResponse>::new(query_string)
            .add_param(tenant_id)
            .add_param(start_date)
            .add_param(end_date)
            .add_option_param(filters.is_include_protocols_mode().map(|_| filters.get_protocols().to_vec()))
            .add_option_param(filters.is_include_endpoints_mode().map(|_| filters.get_endpoints().to_vec()))
            .execute_query(connection_pool).await
    }
}

#[async_trait::async_trait]
impl RequestHandler for NetworkBandwidthRequester {
    async fn request_enveloped_chart(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworkBandwidthRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let filters = request.get_filters();

        let query = QueryBuilder::new(NETWORK_BANDWIDTH_REQUEST_QUERY, 4)
            .add_dynamic_filter(filters.is_include_protocols_mode(), 1, INCLUDE_PROTOCOLS_FILTER_QUERY, EXCLUDE_PROTOCOLS_FILTER_QUERY)
            .add_dynamic_filter(filters.is_include_endpoints_mode(), 1, INCLUDE_ENDPOINT_FILTER_QUERY, EXCLUDE_ENDPOINT_FILTER_QUERY)
            .build_query();

        let executed_query_response = Self::execute_query(
            connection_pool,
            query.as_str(),
            tenant_id,
            request_start_date,
            request_end_date,
            filters,
        ).await?;

        let response: NetworkBandwidthResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            NetworkBandwidthDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthRequestDTO::get_data_type()
    }
}