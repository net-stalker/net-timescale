use sqlx::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;
use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;

const INSERT_NP_QUERY: &str =
    "INSERT INTO Traffic (InsertionTime, TenantId, RawPcapFilePath, ParsedData) VALUES (NOW(), $1, $2, $3)";

pub async fn insert_network_packet_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
    pcap_file_path: &str,
    network_packet: &NetworkPacketDTO
) -> Result<PgQueryResult, Error>
{
    let binary_data: serde_json::Value = match serde_json::from_slice(network_packet.get_network_packet_data()) {
        Ok(data) => data,
        Err(_) => {
            log::error!("Failed to decode network packet data");
            return Err(Error::Decode(Box::new(sqlx::error::Error::Protocol(
                "Failed to decode network packet data".to_string()
            ))))
        }
    };

    sqlx::query(INSERT_NP_QUERY)
        .bind(tenant_id)
        .bind(pcap_file_path)
        .bind(binary_data)
        .execute(&mut **transaction)
        .await
}