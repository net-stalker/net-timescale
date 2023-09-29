use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;


pub async fn create_data_aggregate(con: &Pool<Postgres>)
                                   -> Result<PgQueryResult, Error> {
    sqlx::query("
        CREATE MATERIALIZED VIEW data_aggregate
        WITH (timescaledb.continuous) AS
        SELECT
            time_bucket('1 hour', frame_time) AS bucket,
            group_id,
            agent_id,
            src_addr,
            dst_addr,
            binary_data->'l1'->'frame'->>'frame.protocols' as protocols
        FROM captured_traffic
        GROUP BY bucket, group_id, agent_id, src_addr, dst_addr, protocols;
    ")
        .execute(con)
        .await
}
pub async fn add_refresh_policy_for_data_aggregate(con: &Pool<Postgres>)
                                                   -> Result<PgQueryResult, Error> {
    sqlx::query(
        "SELECT add_continuous_aggregate_policy(
	    'address_pair_aggregate',
	    start_offset => NULL,
	    end_offset => NULL,
	    schedule_interval => INTERVAL '1 minute');"
    )
        .execute(con)
        .await
}