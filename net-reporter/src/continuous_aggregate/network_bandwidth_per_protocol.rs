use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use super::ContinuousAggregate;

pub struct NetworkBandwidthPerProtocolAggregate {}
const CA_NAME: &str = "bandwidth_per_protocol_aggregate";
#[async_trait::async_trait]
impl ContinuousAggregate for NetworkBandwidthPerProtocolAggregate {
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
                    time_bucket('1 hour', frame_time) AS bucket,
                    src_addr,
                    dst_addr,
                    group_id,
                    agent_id,
                    (binary_data->'l1'->'frame'->>'frame.len')::integer as packet_length,
                    binary_data->'l1'->'frame'->>'frame.protocols' as protocols
                FROM captured_traffic
                GROUP BY bucket, src_addr, dst_addr, group_id, agent_id, packet_length, protocols;
            ",
            Self::get_name()
        );
        sqlx::query(query.as_str())
            .execute(pool)
            .await
    }
}