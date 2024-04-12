use std::sync::Arc;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::query::filters::http_overview::response::http_request_method_response::HttpRequestMethodResponse;

const HTTP_REQUEST_METHODS_REQUEST_QUERY: &str = "
    select DISTINCT http->>'http.request.method' as http_request_method
    from http_filters_aggregate, jsonb_path_query(http_part, '$.*') as http
    where
        tenant_id = $1
        AND bucket >= $2
        AND bucket < $3 
        AND http->'http.request.method' is not null
    order by http_request_method
";

#[derive(Default)]
pub struct HttpRequestMethodsRequester {}

impl HttpRequestMethodsRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<HttpRequestMethodResponse>, Error> {
        sqlx::query_as::<Postgres, HttpRequestMethodResponse>(HTTP_REQUEST_METHODS_REQUEST_QUERY)
            .bind(tenant_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await 
    }
}
