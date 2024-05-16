use std::error::Error;
use sqlx::postgres::PgQueryResult;
use sqlx::Postgres;

const CLEAR_QUERY_BUFFER: &str = "
    DELETE FROM Traffic_Buffer
    WHERE Tenant_Id = $1;
";

pub async fn clear_buffer_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(CLEAR_QUERY_BUFFER)
        .bind(tenant_id)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
} 
