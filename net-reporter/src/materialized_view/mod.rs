pub mod http_clients;
pub mod http_overview_filters;
pub mod http_request_methods_distribution;
pub mod http_responses_distribution;
pub mod http_responses;

pub mod network_bandwidth_per_endpoint;
pub mod network_bandwidth_per_protocol;
pub mod network_bandwidth;
pub mod network_graph;
pub mod network_overview_filters;

pub mod total_http_requests;


use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;

#[async_trait::async_trait]
pub trait MaterializedView {
    const CREATE_MATERIALIZED_VIEW_QUERY: &'static str;

    async fn create(
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(Self::CREATE_MATERIALIZED_VIEW_QUERY)
            .execute(pool)
            .await
    }
}