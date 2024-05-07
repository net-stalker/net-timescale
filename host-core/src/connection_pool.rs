use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

pub async fn configure_connection_pool(max_connection_size: u32, connection_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(max_connection_size)
        .connect(connection_url)
        .await
        .unwrap()
}
