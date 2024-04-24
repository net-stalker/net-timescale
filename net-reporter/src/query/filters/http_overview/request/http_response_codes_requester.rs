use std::sync::Arc;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::query::filters::http_overview::response::http_response_code_response::HttpResponseCodeResponse;

const HTTP_RESPONSE_CODES_REQUEST_QUERY: &str = "
SELECT DISTINCT (Http->>'http.response.code')::int8 AS Http_Response_Code
FROM Http_Overview_Filters_Materialized_View, jsonb_path_query(Http_Part, '$.*') AS Http
WHERE
    Tenant_ID = $1
    AND Frametime >= $2
    AND Frametime < $3
    AND Http->'http.response.code' IS NOT NULL
ORDER BY Http_Response_Code;
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
