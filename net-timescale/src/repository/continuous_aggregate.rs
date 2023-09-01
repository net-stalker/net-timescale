use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;


pub async fn create_address_pair_aggregate(con: &Pool<Postgres>)
    -> Result<PgQueryResult, Error> {
    sqlx::query("
        CREATE MATERIALIZED VIEW address_pair_aggregate
        WITH (timescaledb.continuous) AS
        SELECT
            src_addr,
            dst_addr,
            time_bucket('1 minute', frame_time) AS bucket
        FROM captured_traffic
        GROUP BY bucket, src_addr, dst_addr;
    ")
        .execute(con)
        .await
}
pub async fn add_refresh_policy_for_address_pair_aggregate(con: &Pool<Postgres>)
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