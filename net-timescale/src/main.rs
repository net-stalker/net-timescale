use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:PsWDgxZb@localhost").await.unwrap();

    // https://crates.io/crates/sqlx#usage
    let row: (i64, ) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await.unwrap();

    assert_eq!(row.0, 150);
}