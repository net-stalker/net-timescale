use chrono::{DateTime, TimeZone, Utc};
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;

#[derive(sqlx::FromRow, Debug)]
pub struct NetworkPacket {
    pub frame_time: DateTime<Utc>,
    pub src_addr: String,
    pub dst_addr: String,
    pub binary_data: serde_json::Value,
}

impl From<NetworkPacketDTO> for NetworkPacket {
    fn from(value: NetworkPacketDTO) -> NetworkPacket {
        NetworkPacket {
            frame_time: Utc.timestamp_nanos(value.get_frame_time()),
            src_addr: value.get_src_addr().to_string(),
            dst_addr: value.get_dst_addr().to_string(),
            binary_data: serde_json::from_slice(value.get_network_packet_data()).unwrap(),
        }
    }
}

const INSERT_NP_QUERY: &str =
    "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)";

pub async fn insert_network_packet(
    con: &Pool<Postgres>,
    packet: NetworkPacket
) -> Result<PgQueryResult, Error> {
    sqlx::query(INSERT_NP_QUERY)
        .bind(packet.frame_time)
        .bind(packet.src_addr)
        .bind(packet.dst_addr)
        .bind(packet.binary_data)
        .execute(con)
        .await
}

pub async fn insert_network_packet_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    envelope: Envelope
) -> Result<PgQueryResult, Error>
{
    let group_id = envelope.get_jwt_token().ok();
    let agent_id = envelope.get_agent_id().ok();
    
    let envelope_data = envelope.get_data();
    let network_packet: NetworkPacket = NetworkPacketDTO::decode(envelope_data).into();

    sqlx::query("INSERT INTO CAPTURED_TRAFFIC (frame_time, group_id, agent_id, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(network_packet.frame_time)
        .bind(group_id)
        .bind(agent_id)
        .bind(network_packet.src_addr)
        .bind(network_packet.dst_addr)
        .bind(network_packet.binary_data)
        .execute(&mut **transaction)
        .await
}