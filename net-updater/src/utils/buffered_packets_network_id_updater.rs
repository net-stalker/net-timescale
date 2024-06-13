use std::error::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

const UPDATE_BUFFERED_PACKETS_NETWORK_ID_QUERY: &str = "
    UPDATE Traffic_Buffer
    SET Network_ID = $1
    WHERE Pcap_ID IN (SELECT UNNEST($2)) AND Tenant_Id = $3;
";

pub async fn update_buffered_packets_network_id_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    network_id: Option<&str>,
    packets_ids: &[&str],
    tenant_id: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(UPDATE_BUFFERED_PACKETS_NETWORK_ID_QUERY)
        .bind(network_id)
        .bind(packets_ids)
        .bind(tenant_id)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
