use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;

pub trait MaterializedViewQueries: Send + Sync {
    fn get_name(&self) -> String;

    fn get_creation_query(&self) -> String;

    fn get_refresh_query_blocking(&self) -> String {
        format!("REFRESH MATERIALIZED VIEW {};", self.get_name())
    }

    fn get_refresh_query_concurrent(&self) -> String {
        format!("REFRESH MATERIALIZED VIEW CONCURRENTLY {};", self.get_name())
    }
}

#[async_trait::async_trait]
pub trait MaterializedView: MaterializedViewQueries {

    async fn create(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let create_query = self.get_creation_query();
        sqlx::query(&create_query)
            .execute(pool)
            .await
    }

    async fn refresh_blocking(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let refresh_query = self.get_refresh_query_blocking();
        sqlx::query(&refresh_query)
            .execute(pool)
            .await
    }

    async fn refresh_concurrently(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let refresh_query = self.get_refresh_query_concurrent();
        sqlx::query(&refresh_query)
            .execute(pool)
            .await
    }
}