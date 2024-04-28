use net_inserter_api::api::network::InsertNetworkRequestDTO;
use std::error::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

const INSERT_NETWORK_QUERY: &str = 
    "INSERT INTO Networks (Network_Name, Tenant_Id, Network_Color) VALUES ($1, $2, $3)";

pub async fn insert_network_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
    network: &InsertNetworkRequestDTO
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>>
{
    let res = sqlx::query(INSERT_NETWORK_QUERY)
        .bind(network.get_name().to_string())
        .bind(tenant_id)
        .bind(network.get_color().to_string())
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
