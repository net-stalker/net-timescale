use std::error::Error;
use sqlx::postgres::PgQueryResult;
use sqlx::Postgres;

const INSERT_NP_QUERY: &str = "
    INSERT INTO Traffic (Pcap_ID, Insertion_Time, Tenant_Id, Raw_Pcap_File_Path, Parsed_Data) VALUES ($1, NOW(), $2, $3, $4)
";

pub async fn insert_network_packet_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    packet_id: &str,
    tenant_id: &str,
    pcap_file_path: &str,
    network_packet_data: &[u8],
) -> Result<PgQueryResult, Box<dyn Error + Sync + Send>> {
    let binary_data: serde_json::Value = match serde_json::from_slice(network_packet_data) {
        Ok(data) => data,
        Err(_) => {
            let error_message = "Failed to decode network packet data into json value";
            log::error!("{error_message}");
            return Err(error_message.into())
        }
    };
    let res = sqlx::query(INSERT_NP_QUERY)
        .bind(packet_id)
        .bind(tenant_id)
        .bind(pcap_file_path)
        .bind(binary_data)
        .execute(&mut **transaction)
        .await?;
    Ok(res)
}
