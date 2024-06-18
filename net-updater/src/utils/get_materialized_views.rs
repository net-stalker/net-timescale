use std::error::Error;
use sqlx::Postgres;
use sqlx::Row;

const GET_MATERIALIZED_VIEWS_QUERY: &str = "
    SELECT matviewname
    FROM pg_matviews;
";

pub async fn get_materialized_views_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
) -> Result<Vec<String>, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(GET_MATERIALIZED_VIEWS_QUERY)
        .fetch_all(&mut **transaction)
        .await?
        .into_iter()
        .map(|row| row.get(0))
        .collect();
    Ok(res)
}
