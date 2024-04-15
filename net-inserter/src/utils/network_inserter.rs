use net_inserter_api::api::network::InsertNetworkRequestDTO;
use sqlx::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

const INSERT_NETWORK_QUERY: &str = "TODO: write a valid query after fixing migrations";

pub async fn insert_network_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
    agent_id: &str,
    network: &InsertNetworkRequestDTO
) -> Result<PgQueryResult, Error>
{
    sqlx::query(INSERT_NETWORK_QUERY)
        .bind(tenant_id)
        .bind(agent_id)
        .bind(network.get_name().to_string())
        .bind(network.get_color().to_string())
        .execute(&mut **transaction)
        .await
}
