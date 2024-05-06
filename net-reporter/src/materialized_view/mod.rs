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
pub trait MaterializedViewQueries {
    const NAME: &'static str;

    fn get_creation_query() -> String;

    fn get_refresh_query_blocking() -> String {
        format!("REFRESH MATERIALIZED VIEW {};", Self::NAME)
    }

    fn get_refresh_query_concurrent() -> String {
        format!("REFRESH MATERIALIZED VIEW CONCURRENTLY {};", Self::NAME)
    }
}

#[async_trait::async_trait]
pub trait MaterializedView: MaterializedViewQueries {

    async fn create(
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let create_query = Self::get_creation_query();
        sqlx::query(&create_query)
            .execute(pool)
            .await
    }

    async fn refresh_blocking(
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let refresh_query = Self::get_refresh_query_blocking();
        sqlx::query(&refresh_query)
            .execute(pool)
            .await
    }

    async fn refresh_concurrently(
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let refresh_query = Self::get_refresh_query_concurrent();
        sqlx::query(&refresh_query)
            .execute(pool)
            .await
    }
}