use sqlx::{Pool, Postgres};

pub async fn establish_connection() -> Pool<Postgres> {
    let database_url = "postgres://postgres:PsWDgxZb@localhost:5433".to_owned();
    Pool::<Postgres>::connect(database_url.as_str()).await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
