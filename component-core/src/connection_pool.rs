use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn configure_connection_pool(connection_url: &str, max_connection_size: u32) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(max_connection_size)
        .connect(connection_url)
        .await
        .unwrap()
}
