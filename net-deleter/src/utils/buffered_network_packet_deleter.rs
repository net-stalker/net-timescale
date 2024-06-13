use std::error::Error;

use sqlx::postgres::PgQueryResult;
use sqlx::Postgres;

const DELETE_NP_QUERY_BUFFER: &str = "
    DELETE FROM Traffic_Buffer
    WHERE Pcap_ID = $1 AND Tenant_Id = $2;
";

pub async fn delete_network_packet_buffer_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    packet_id: &str,
    tenant_id: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(DELETE_NP_QUERY_BUFFER)
        .bind(packet_id)
        .bind(tenant_id)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
