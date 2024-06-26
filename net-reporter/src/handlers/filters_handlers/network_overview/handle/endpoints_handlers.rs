use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Postgres;
use sqlx::Transaction;

use crate::handlers::filters_handlers::network_overview::response::endpoint_response::EndpointResponse;

const ENDPOINTS_REQUEST_QUERY: &str = "
    SELECT DISTINCT Src_IP AS Endpoint
    FROM Network_Overview_Filters_Materialized_View
    WHERE
        Tenant_ID = $1
        AND Frametime >= $2
        AND Frametime < $3
    UNION
    SELECT DISTINCT Dst_IP AS Endpoint
    FROM Network_Overview_Filters_Materialized_View
    WHERE
        Tenant_ID = $1
        AND Frametime >= $2
        AND Frametime < $3
    ORDER BY Endpoint;
";

#[derive(Default)]
pub struct EndpointsHandler {}

impl EndpointsHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        transcation: &mut Transaction<'_, Postgres>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<EndpointResponse>, Error> {
        sqlx::query_as(ENDPOINTS_REQUEST_QUERY)
            .bind(tenant_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&mut **transcation)
            .await
    }
}
