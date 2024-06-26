use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Postgres;
use sqlx::Transaction;

use crate::handlers::filters_handlers::network_overview::response::protocol_response::ProtocolResponse;

const PROTOCOLS_REQUEST_QUERY: &str = "
    SELECT DISTINCT
        unnest(string_to_array(Protocols, ':')) AS protocol
    FROM Network_Overview_Filters_Materialized_View
    WHERE
        Tenant_ID = $1
        AND Frametime >= $2
        AND Frametime < $3
    ORDER BY protocol;
";

#[derive(Default)]
pub struct ProtocolsHandler {}

impl ProtocolsHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        transcation: &mut Transaction<'_, Postgres>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<ProtocolResponse>, Error> {
        sqlx::query_as(PROTOCOLS_REQUEST_QUERY)
            .bind(tenant_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&mut **transcation)
            .await
    }
}
