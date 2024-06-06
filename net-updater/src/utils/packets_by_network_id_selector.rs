use std::error::Error;
use sqlx::Postgres;

const SELECT_PACKETS_BY_NETWORK_ID_QUERY: &str = "
    SELECT Pcap_ID, Raw_Pcap_File_Path FROM Traffic
    WHERE Network_ID = $1 AND Tenant_Id = $2
    ORDER BY (Parsed_Data->'l1'->'frame'->>'frame.time_epoch')::DECIMAL;
";

pub async fn select_packets_by_network_id_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    network_id: &str,
    tenant_id: &str,
) -> Result<Vec<PcapPathInfo>, Box<dyn Error + Sync + Send>> {
    let res = sqlx::query_as(SELECT_PACKETS_BY_NETWORK_ID_QUERY)
        .bind(network_id)
        .bind(tenant_id)
        .fetch_all(&mut **transaction)
        .await?;
    Ok(res)
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct PcapPathInfo {
    #[sqlx(rename = "Pcap_ID")]
    pub id: String,
    #[sqlx(rename = "Raw_Pcap_File_Path")]
    pub pcap_file_path: String,
}
