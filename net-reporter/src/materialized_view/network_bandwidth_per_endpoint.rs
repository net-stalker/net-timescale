use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;

use super::MaterializedView;

const CA_NAME: &str = "Network_Bandwidth_Per_Endpoint_Materialized_View";
const CREATE_MATERIALIZED_VIEW: &str = &format!("
CREATE MATERIALIZED VIEW IF NOT EXISTS {}
AS
SELECT
    Parsed_Data ->'l1'->'frame'->>'frame.time' AS Frametime,
    Tenant_ID,
    Network_ID,
    Parsed_Data->'l3'->'ip'->>'ip.src' as Src_IP,
    Parsed_Data->'l3'->'ip'->>'ip.dst' as Dst_IP,
    Parsed_Data->'l1'->'frame'->>'frame.len' as Packet_Length,
    Parsed_Data->'l1'->'frame'->>'frame.protocols' as Protocols
FROM Traffic
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Protocols;",
CA_NAME);

pub struct NetworkBandwidthPerEndpointMaterializedView {}

#[async_trait::async_trait]
impl MaterializedView for NetworkBandwidthPerEndpointMaterializedView {
    async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        // TODO: investigate using binds in sqlx to remove formatting string #8692yh6n4
        sqlx::query(CREATE_MATERIALIZED_VIEW)
            .execute(pool)
            .await
    }
}