use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

#[async_trait::async_trait]
pub trait ContinuousAggregate {
    fn get_name() -> &'static str;
    async fn create(pool: &sqlx::Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error>;
    async fn add_refresh_policy(
        pool: &sqlx::Pool<Postgres>,
        start_offset: Option<&str>,
        end_offset: Option<&str>,
        schedule_interval: &str
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(
            "SELECT add_continuous_aggregate_policy(
	    ? ,
	    start_offset => NULL ,
	    end_offset => NULL ,
	    schedule_interval => INTERVAL ?);"
        )
            .bind(Self::get_name())
            // .bind(start_offset)
            // .bind(end_offset)
            .bind(schedule_interval)
            .execute(pool)
            .await
    }
}

pub mod network_graph;
pub mod bandwidth_per_endpoint;
