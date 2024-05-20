use std::error::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;

const UPDATE_PACKETS_PARSED_DATA_QUERY: &str = "
    UPDATE Traffic
    SET Parsed_Data = $2
    WHERE Pcap_ID = $1 AND Tenant_Id = $3;
";

pub async fn update_packets_parsed_data_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    packet_id: &str,
    parsed_data: &serde_json::Value,
    tenant_id: &str,
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query(UPDATE_PACKETS_PARSED_DATA_QUERY)
        .bind(packet_id)
        .bind(parsed_data)
        .bind(tenant_id)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
