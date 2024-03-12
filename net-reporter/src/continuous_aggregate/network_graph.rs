use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;

use super::ContinuousAggregate;

pub struct NetworkGraphAggregate {}

const CA_NAME: &str = "network_graph_aggregate";
#[async_trait::async_trait]
impl ContinuousAggregate for NetworkGraphAggregate {
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
                    group_id,
                    agent_id,
                    src_addr,
                    dst_addr,
                    (binary_data->'l1'->'frame'->>'frame.len')::integer AS packet_length,
                    binary_data->'l1'->'frame'->>'frame.protocols' as protocols
                FROM captured_traffic
                GROUP BY bucket, group_id, agent_id, src_addr, dst_addr, packet_length, protocols;
            ",
            Self::get_name()
        );
        sqlx::query(query.as_str())
            .bind(Self::get_name())
            .execute(pool)
            .await
    }
}