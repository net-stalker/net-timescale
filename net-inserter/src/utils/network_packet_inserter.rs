use chrono::TimeZone;
use chrono::Utc;
use sqlx::Error;
use sqlx::Postgres;
use sqlx::postgres::PgQueryResult;
use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;

const INSERT_NP_QUERY: &str =
    "INSERT INTO CAPTURED_TRAFFIC (frame_time, group_id, agent_id, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4, $5, $6)";

pub async fn insert_network_packet_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
    agent_id: &str,
    network_packet: &NetworkPacketDTO
) -> Result<PgQueryResult, Error>
{
    let binary_data: serde_json::Value = match serde_json::from_slice(network_packet.get_network_packet_data()) {
        Ok(data) => data,
        Err(_) => return Err(Error::Decode(Box::new(sqlx::error::Error::Protocol(
            "Failed to decode network packet data".to_string()
        ))))
    };

    sqlx::query(INSERT_NP_QUERY)
        .bind(Utc.timestamp_nanos(network_packet.get_frame_time()))
        .bind(tenant_id)
        .bind(agent_id)
        .bind(network_packet.get_src_addr().to_string())
        .bind(network_packet.get_dst_addr().to_string())
        .bind(binary_data)
        .execute(&mut **transaction)
        .await
}