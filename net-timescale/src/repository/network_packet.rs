use chrono::{DateTime, TimeZone, Utc};
use sqlx::{Acquire, Database, Error, Pool, Postgres};
use sqlx::postgres::{PgConnection, PgQueryResult};
use net_timescale_api::api::network_packet::NetworkPacketDTO;

#[derive(sqlx::FromRow, Debug)]
pub struct NetworkPacket {
    pub frame_time: DateTime<Utc>,
    pub src_addr: String,
    pub dst_addr: String,
    pub binary_data: serde_json::Value,
}
impl Into<NetworkPacket> for NetworkPacketDTO {
    fn into(self) -> NetworkPacket {
        NetworkPacket {
            frame_time: Utc.timestamp_nanos(self.get_frame_time()),
            src_addr: self.get_src_addr().to_string(),
            dst_addr: self.get_dst_addr().to_string(),
            binary_data: serde_json::from_slice(self.get_network_packet_data()).unwrap(),
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
    packet: NetworkPacket
) -> Result<PgQueryResult, Error>
{
    sqlx::query(INSERT_NP_QUERY)
        .bind(packet.frame_time)
        .bind(packet.src_addr)
        .bind(packet.dst_addr)
        .bind(packet.binary_data)
        .execute(&mut **transaction)
        .await
}
