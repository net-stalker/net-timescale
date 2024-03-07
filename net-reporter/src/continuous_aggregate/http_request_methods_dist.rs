use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use super::ContinuousAggregate;

pub struct HttpRequestMethodsDistAggregate {}
const CA_NAME: &str = "http_request_methods_dist";
#[async_trait::async_trait]
impl ContinuousAggregate for HttpRequestMethodsDistAggregate {
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
                    src_addr,
                    dst_addr,
                    (binary_data->'l1'->'frame'->>'frame.len')::integer as packet_length,
                    binary_data->'l5'->'http' as http_part
                FROM captured_traffic
                where
                    binary_data->'l5'->'http' is not null
                    and (binary_data->'l5'->'http'->>'http.request')::bool
                GROUP BY bucket, group_id, agent_id, src_addr, dst_addr, packet_length, binary_data;
            ",
            Self::get_name()
        );
        sqlx::query(query.as_str())
            .execute(pool)
            .await
    }
}