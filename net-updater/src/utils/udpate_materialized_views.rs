use std::error::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

pub async fn udpate_materialized_view_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    view_name: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(format!("REFRESH MATERIALIZED VIEW {};", view_name).as_str())
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
