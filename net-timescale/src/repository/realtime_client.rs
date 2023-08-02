use chrono::{DateTime, Utc};
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgConnection;
use futures::stream::BoxStream;


pub async fn update_client_id(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    client_id: i64
) {
    todo!()
}