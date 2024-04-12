use std::sync::Arc;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::query::filters::http_overview::response::endpoint_response::EndpointResponse;

const ENDPOINTS_REQUEST_QUERY: &str = "
    SELECT DISTINCT src_addr AS endpoint
    FROM http_filters_aggregate
    WHERE
        tenant_id = $1
        AND bucket >= $2
        AND bucket < $3
    UNION
    SELECT DISTINCT dst_addr AS endpoint
    FROM http_filters_aggregate
    WHERE
        tenant_id = $1
        AND bucket >= $2
        AND bucket < $3
    order by endpoint;
";

#[derive(Default)]
pub struct EndpointsRequester {}

impl EndpointsRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<EndpointResponse>, Error> {
        sqlx::query_as::<Postgres, EndpointResponse>(ENDPOINTS_REQUEST_QUERY)
            .bind(tenant_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await 
    }
}
