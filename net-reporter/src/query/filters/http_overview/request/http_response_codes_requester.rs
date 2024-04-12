use std::sync::Arc;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::query::filters::http_overview::response::http_response_code_response::HttpResponseCodeResponse;

const HTTP_RESPONSE_CODES_REQUEST_QUERY: &str = "
    select DISTINCT (http->>'http.response.code')::int8 as http_response_code
    from http_filters_aggregate, jsonb_path_query(http_part, '$.*') as http
    where
        tenant_id = $1
        AND bucket >= $2
        AND bucket < $3
        AND http->'http.response.code' is not null
    order by http_response_code;
";

#[derive(Default)]
pub struct HttpResponseCodesRequester {}

impl HttpResponseCodesRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<HttpResponseCodeResponse>, Error> {
        sqlx::query_as::<Postgres, HttpResponseCodeResponse>(HTTP_RESPONSE_CODES_REQUEST_QUERY)
            .bind(tenant_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await 
    }
}
