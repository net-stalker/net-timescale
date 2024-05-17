use std::error::Error;
use sqlx::postgres::PgQueryResult;
use sqlx::Postgres;

const DELETE_NETWORK_QUERY: &str = "
    DELETE FROM Networks
    WHERE Network_ID = $1 AND Tenant_Id = $2;
";

pub async fn delete_network_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    network_id: &str,
    tenant_id: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(DELETE_NETWORK_QUERY)
        .bind(network_id)
        .bind(tenant_id)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
