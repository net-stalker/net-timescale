use std::error::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

const BUFFER_FLUSH_QUERY: &str = 
    "INSERT INTO Traffic SELECT * FROM Traffic_Buffer WHERE Tenant_Id = $1;";

pub async fn flush_buffer_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(BUFFER_FLUSH_QUERY)
        .bind(tenant_id)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
