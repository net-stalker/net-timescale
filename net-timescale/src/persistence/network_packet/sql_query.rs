use diesel::pg::Pg;
use chrono::{DateTime, Utc, TimeZone};
use diesel::query_builder::{AstPass, QueryFragment, SqlQuery};
use diesel::{IntoSql, QueryResult, sql_query};
use diesel::sql_types::{Jsonb, Text, Timestamptz};
use net_timescale_api::api::network_packet::NetworkPacketDTO;

// TODO: think about creating macros to make it possible to write desirable queries in there
// like in mybatis
// #[derive(diesel::QueryId)]
pub struct NetworkPacket {
    frame_time: DateTime<Utc>,
    src_addr: String,
    dst_addr: String,
    binary_data: serde_json::Value,
}

impl NetworkPacket {
    pub fn new(dto: NetworkPacketDTO) -> Self {
        NetworkPacket {
            frame_time: Utc.timestamp_millis_opt(dto.get_frame_time()).unwrap(),
            src_addr: dto.get_src_addr().to_string(),
            dst_addr: dto.get_dst_addr().to_string(),
            binary_data: serde_json::from_slice(&*dto.get_network_packet_data()).unwrap(),
        }
    }
}


impl crate::persistence::sql_query::SqlQuery for NetworkPacket {
    fn get_sql_query(self) -> SqlQuery {
        let query = sql_query("INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)");
        let query= query
            .bind::<Timestamptz, _>(self.frame_time)
            .bind::<Text, _>(self.src_addr)
            .bind::<Text, _>(self.dst_addr)
            .bind::<Jsonb, _>(self.binary_data);
        let query = query.bind::<SqlQuery, _>(&query);
        query.into_sql::<SqlQuery>()
    }
}

#[cfg(test)]
mod tests {
    // TODO: add tests
}