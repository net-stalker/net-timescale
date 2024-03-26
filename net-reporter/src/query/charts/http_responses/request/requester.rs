use std::sync::Arc;

use net_reporter_api::api::http_responses::http_responses::HttpResponsesDTO;
use net_reporter_api::api::http_responses::http_responses_filters::HttpResponsesFiltersDTO;
use net_token_verifier::fusion_auth::jwt_token::Jwt;
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

use crate::query::charts::http_responses::response::http_response::HttpResponseResponse;
use crate::query::charts::http_responses::response::http_responses::HttpResponsesResponse;
use crate::query::requester::Requester;
use crate::query_builder::query_builder::QueryBuilder;
use crate::query_builder::sqlx_query_builder_wrapper::SqlxQueryBuilderWrapper;

use net_reporter_api::api::http_responses::http_responses_request::HttpResponsesRequestDTO;

const EXCLUDE_HTTP_METHODS_FILTER_QUERY: &str = "
    AND ((http->>'http.response.code')::int8 NOT IN (SELECT unnest({})) AND (http->>'http.response.code')::int8 NOT IN (SELECT unnest({})))
";

const INCLUDE_HTTP_METHODS_FILTER_QUERY: &str = "
    AND ((http->>'http.response.code')::int8 IN (SELECT unnest({})) OR (http->>'http.response.code')::int8 IN (SELECT unnest({})))
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr IN (SELECT unnest({})) OR dst_addr IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "   
    AND (src_addr NOT IN (SELECT unnest({})) AND dst_addr NOT IN (SELECT unnest({})))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND packet_length >= {}
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND packet_length < {}
";

const HTTP_RESPONSES_REQUEST_QUERY: &str = "
    SELECT
        (http_part->>'http.date')::timestamptz AS http_date,
        packet_date::timestamptz as packet_date,
        src_addr AS client,
        dst_addr AS server,
        (http->>'http.response.code')::int8 AS response_code
    FROM http_responses_aggregate, jsonb_path_query(http_part, '$.*') AS http
    WHERE
        group_id = $1
        AND bucket >= $2
        AND bucket < $3
        AND http->'http.response.code' IS NOT NULL
        {}
        {}
        {}
        {}
";

#[derive(Default)]
pub struct HttpResponsesRequester {}

impl HttpResponsesRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        query_string: &str,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &HttpResponsesFiltersDTO,
    ) -> Result<Vec<HttpResponseResponse>, Error> {
        log::info!("Query Parameters: {:?}", group_id);
        log::info!("Query Parameters: {}", start_date);
        log::info!("Query Parameters: {}", end_date);
        SqlxQueryBuilderWrapper::<HttpResponseResponse>::new(query_string)
            .add_option_param(group_id.map(|group_id| group_id.to_string()))
            .add_param(start_date)
            .add_param(end_date)
            .add_option_param(filters.is_include_http_methods_mode().map(|_| filters.get_http_responses().to_vec()))
            .add_option_param(filters.is_include_endpoints_mode().map(|_| filters.get_endpoints().to_vec()))
            .add_option_param(filters.get_bytes_lower_bound())
            .add_option_param(filters.get_bytes_upper_bound())
            .execute_query(connection_pool).await
    }
}

#[async_trait::async_trait]
impl Requester for HttpResponsesRequester {
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
        let request = HttpResponsesRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let filters = request.get_filters();

        let query = QueryBuilder::new(HTTP_RESPONSES_REQUEST_QUERY, 4)
            .add_dynamic_filter(filters.is_include_http_methods_mode(), 1, INCLUDE_HTTP_METHODS_FILTER_QUERY, EXCLUDE_HTTP_METHODS_FILTER_QUERY)
            .add_dynamic_filter(filters.is_include_endpoints_mode(), 1, INCLUDE_ENDPOINT_FILTER_QUERY, EXCLUDE_ENDPOINT_FILTER_QUERY)
            .add_static_filter(filters.get_bytes_lower_bound(), SET_LOWER_BYTES_BOUND, 1)
            .add_static_filter(filters.get_bytes_upper_bound(), SET_UPPER_BYTES_BOUND, 1)
            .build_query();

        let executed_query_response = Self::execute_query(
            connection_pool,
            query.as_str(),
            jwt.get_tenant_id(),
            request_start_date,
            request_end_date,
            filters,
        ).await;

        if let Err(e) = executed_query_response {
            return Err(format!("error: {:?}", e));
        }
        let executed_query_response = executed_query_response.unwrap();

        let response: HttpResponsesResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: HttpResponsesDTO = response.into();

        Ok(Envelope::new(
            enveloped_request.get_jwt_token().ok(),
            request_agent_id,
            HttpResponsesDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        HttpResponsesRequestDTO::get_data_type()
    }
}