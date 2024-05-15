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

use net_reporter_api::api::http_responses_distribution::http_responses_distribution::HttpResponsesDistributionDTO;
use net_reporter_api::api::http_responses_distribution::http_responses_distribution_request::HttpResponsesDistributionRequestDTO;
use net_reporter_api::api::http_responses_distribution::http_responses_disribution_filters::HttpResponsesDistributionFiltersDTO;

use crate::handlers::chart_handlers::http_responses_distribution::response::http_responses_distribution::HttpResponsesDistributionResponse;
use crate::handlers::chart_handlers::http_responses_distribution::response::http_responses_distribution_bucket::HttpResponsesDistributionBucketResponse;

use crate::query_builder::query_builder::QueryBuilder;
use crate::query_builder::sqlx_query_builder_wrapper::SqlxQueryBuilderWrapper;

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (Src_IP IN (SELECT unnest({})) OR Dst_IP IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (Src_IP NOT IN (SELECT unnest({})) AND Dst_IP NOT IN (SELECT unnest({})))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND SUM(Packet_Length) >= {}
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND SUM(Packet_Length) < {}
";

const HTTP_RESPONSES_DIST_REQUEST_QUERY: &str = "
    SELECT Frametime, (http->>'http.response.code')::int8 AS Response_Code, COUNT(Http_Part) AS Amount
    FROM Http_Responses_Distribution_Materialized_View, jsonb_path_query(Http_Part, '$.*') AS Http
    WHERE
        Tenant_ID = $1
        AND Frametime >= $2
        AND Frametime < $3
        AND Http->'http.response.code' IS NOT NULL
        {}
    GROUP BY
        Frametime, Http, Http->'http.response.code'
    HAVING
        1 = 1
        {}
        {}
    ORDER BY Frametime;
";

#[derive(Default)]
pub struct HttpResponsesDistributionHandler {}

impl HttpResponsesDistributionHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        query_string: &str,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &HttpResponsesDistributionFiltersDTO,
    ) -> Result<Vec<HttpResponsesDistributionBucketResponse>, Error> {
        SqlxQueryBuilderWrapper::<HttpResponsesDistributionBucketResponse>::new(query_string)
            .add_param(tenant_id)
            .add_param(start_date)
            .add_param(end_date)
            .add_option_param(filters.is_include_endpoints_mode().map(|_| filters.get_endpoints().to_vec()))
            .add_option_param(filters.get_bytes_lower_bound())
            .add_option_param(filters.get_bytes_upper_bound())
            .execute_query(connection_pool).await
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for HttpResponsesDistributionHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = HttpResponsesDistributionRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let request_filters = request.get_filters();

        let query = QueryBuilder::new(HTTP_RESPONSES_DIST_REQUEST_QUERY, 4)
            .add_dynamic_filter(request_filters.is_include_endpoints_mode(), 1, INCLUDE_ENDPOINT_FILTER_QUERY, EXCLUDE_ENDPOINT_FILTER_QUERY)
            .add_static_filter(request_filters.get_bytes_lower_bound(), SET_LOWER_BYTES_BOUND, 1)
            .add_static_filter(request_filters.get_bytes_upper_bound(), SET_UPPER_BYTES_BOUND, 1)
            .build_query();

        let executed_query_response = Self::execute_query(
            connection_pool,
            query.as_str(),
            tenant_id,
            request_start_date,
            request_end_date,
            request_filters,
        ).await?;

        let response: HttpResponsesDistributionResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: HttpResponsesDistributionDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            HttpResponsesDistributionDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_handler_type(&self) -> String {
        HttpResponsesDistributionRequestDTO::get_data_type().to_string()
    }
}