use std::sync::Arc;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::handlers::filters_handlers::http_overview::response::endpoint_response::EndpointResponse;

const ENDPOINTS_REQUEST_QUERY: &str = "
SELECT DISTINCT Src_IP AS Endpoint
FROM Http_Overview_Filters_Materialized_View
WHERE
    Tenant_ID = $1
    AND Frametime >= $2
    AND Frametime < $3
    AND Network_ID = $4
UNION
SELECT DISTINCT Dst_IP AS Endpoint
FROM Http_Overview_Filters_Materialized_View
WHERE
    Tenant_ID = $1
    AND Frametime >= $2
    AND Frametime < $3
    AND Network_ID = $4
ORDER BY Endpoint;
";

#[derive(Default)]
pub struct EndpointsHandler {}

impl EndpointsHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        network_id: &str,
    ) -> Result<Vec<EndpointResponse>, Error> {
        sqlx::query_as::<Postgres, EndpointResponse>(ENDPOINTS_REQUEST_QUERY)
            .bind(tenant_id)
            .bind(start_date)
            .bind(end_date)
            .bind(network_id)
            .fetch_all(connection_pool.as_ref())
            .await 
    }
}
