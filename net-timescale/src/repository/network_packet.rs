use chrono::{DateTime, TimeZone, Utc};
use diesel::{PgConnection, QueryResult, RunQueryDsl, sql_query};
use diesel::sql_types::{Timestamptz, Text, Jsonb};
use net_timescale_api::api::network_packet::NetworkPacketDTO;


pub struct NetworkPacket {
    frame_time: DateTime<Utc>,
    src_addr: String,
    dst_addr: String,
    binary_data: serde_json::Value,
}
impl Into<NetworkPacket> for NetworkPacketDTO {
    fn into(self) -> NetworkPacket {
        NetworkPacket {
            frame_time: Utc.timestamp_millis_opt(self.get_frame_time()).unwrap(),
            src_addr: self.get_src_addr().to_string(),
            dst_addr: self.get_dst_addr().to_string(),
            binary_data: serde_json::from_slice(self.get_network_packet_data()).unwrap(),
        }
    }
}

pub fn insert_network_packet(con: &mut PgConnection, packet: NetworkPacket) -> QueryResult<usize> {
    let query = sql_query("INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
    query
        .bind::<Timestamptz, _>(packet.frame_time)
        .bind::<Text, _>(packet.src_addr)
        .bind::<Text, _>(packet.dst_addr)
        .bind::<Jsonb, _>(packet.binary_data)
        .execute(con)
}