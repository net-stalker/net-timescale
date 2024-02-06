use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use super::ContinuousAggregate;

pub struct NetworkBandwidthAggregate {}
const CA_NAME: &str = "network_bandwidth_aggregate";
#[async_trait::async_trait]
impl ContinuousAggregate for NetworkBandwidthAggregate {
    fn get_name() -> &'static str {
        CA_NAME
    }

    async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        // TODO: investigate using binds in sqlx to remove formatting string #8692yh6n4
        let query = format!(
            "
                CREATE MATERIALIZED VIEW {}
                WITH (timescaledb.continuous) AS
                SELECT
                    time_bucket('2 minutes', frame_time) AS bucket,
                    group_id,
                    agent_id,
                    (binary_data->'l1'->'frame'->>'frame.len')::integer AS packet_length
                FROM captured_traffic
                GROUP BY bucket, group_id, agent_id, packet_length;
            ",
            Self::get_name()
        );
        sqlx::query(query.as_str())
            .execute(pool)
            .await
    }
}