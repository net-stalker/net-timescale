use std::error::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

const UPDATE_NETWORK_QUERY: &str = "
    UPDATE Networks
    SET Network_Name = $1, Network_Color = $2
    WHERE Network_Id = $3 AND Tenant_Id = $4;
";

pub async fn update_network_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    name: &str,
    color: &str,
    network_id: &str,
    tenant_id: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(UPDATE_NETWORK_QUERY)
        .bind(name)
        .bind(color)
        .bind(network_id)
        .bind(tenant_id)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
