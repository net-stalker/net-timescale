use std::sync::Arc;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::query::filters::http_overview::response::http_request_method_response::HttpRequestMethodResponse;

const HTTP_REQUEST_METHODS_REQUEST_QUERY: &str = "
SELECT DISTINCT Http->>'http.request.method' as Http_Request_Method
FROM Http_Overview_Filters_Materialized_View, jsonb_path_query(Http_Part, '$.*') as Http
WHERE
    Tenant_ID = $1
    AND Frametime >= $2
    AND Frametime < $3 
    AND Http_Part->'http.request.method' IS NOT NULL
ORDER BY Http_Request_Method
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
