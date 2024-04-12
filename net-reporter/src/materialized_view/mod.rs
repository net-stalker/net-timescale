pub mod manager;

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


use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

#[async_trait::async_trait]
pub trait MaterializedView {
    async fn create(pool: &sqlx::Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error>;
}