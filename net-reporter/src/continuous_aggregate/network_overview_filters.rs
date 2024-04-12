use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;

use super::ContinuousAggregate;

pub struct NetworkOverviewFiltersAggregate { }

const CA_NAME: &str = "network_overview_filters";

#[async_trait::async_trait]
impl ContinuousAggregate for NetworkOverviewFiltersAggregate {
    fn get_name() -> &'static str {
        CA_NAME
    }

    async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        let query = format!(
            "
                CREATE MATERIALIZED VIEW {}
                WITH (timescaledb.continuous) AS
                SELECT
                    time_bucket('2 minutes', frame_time) AS bucket,
                    tenant_id,
                    src_addr,
                    dst_addr,
                    (binary_data->'l1'->'frame'->>'frame.len')::integer AS packet_length,
                    binary_data->'l1'->'frame'->>'frame.protocols' as protocols
                FROM captured_traffic
                GROUP BY bucket, tenant_id, src_addr, dst_addr, packet_length, protocols;
            ",
            Self::get_name()
        );
        sqlx::query(query.as_str())
            .execute(pool)
            .await
    }
}