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
        // TODO: investigate using binds in sqlx to remove formatting string #8692yh6n4
        let start_offset = match start_offset {
            None => "NULL",
            Some(interval) => interval
        };
        let end_offset = match end_offset {
            None => "NULL",
            Some(interval) => interval
        };
        let query = format!(
                "SELECT add_continuous_aggregate_policy(
                '{}',
                start_offset => {},
                end_offset => {},
                schedule_interval => INTERVAL '{}'
            );",
            Self::get_name(),
            start_offset,
            end_offset,
            schedule_interval,
        );
        sqlx::query(query.as_str())
            .execute(pool)
            .await
    }
}

pub mod network_graph;
pub mod bandwidth_per_endpoint;
pub mod http_clients;
pub mod http_request_methods_distribution;
pub mod http_responses_dist;
pub mod http_responses;
pub mod network_bandwidth_per_protocol;
pub mod network_bandwidth;
pub mod network_overview_filters;
pub mod total_http_requests;
