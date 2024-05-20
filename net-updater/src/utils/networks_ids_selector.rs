use std::error::Error;
use sqlx::Postgres;
use sqlx::Row;

const SELECT_NETWORKS_IDS_QUERY: &str = "
    SELECT Network_ID FROM Networks
    WHERE Tenant_ID = $1;
";

pub async fn select_networks_ids_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
) -> Result<Vec<String>, Box<dyn Error + Sync + Send>> {
    let res: Vec<String> = sqlx::query(SELECT_NETWORKS_IDS_QUERY)
        .bind(tenant_id)
        .fetch_all(&mut **transaction)
        .await?
        .into_iter()
        .map(|row| row.get("Network_ID"))
        .collect();
    Ok(res)
}
