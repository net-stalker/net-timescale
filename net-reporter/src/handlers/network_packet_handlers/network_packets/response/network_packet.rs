use net_reporter_api::api::network_packet::network_packet::NetworkPacketDTO;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

#[derive(sqlx::FromRow, Debug)]
pub struct NetworkPacket {
    pub id: String,
    pub network_id: Option<String>,
    pub insertion_time: DateTime<Utc>,
    pub src: String,
    pub dst: String,
    pub protocols: Vec<String>,
    pub json_data: serde_json::Value,
}

impl From<NetworkPacket> for NetworkPacketDTO {
    fn from(value: NetworkPacket) -> Self {
        NetworkPacketDTO::new(
            &value.id,
            value.network_id.as_deref(),
            value.insertion_time.timestamp_nanos_opt().unwrap(),
            &value.src,
            &value.dst,
            &value.protocols,
            &serde_json::to_vec(&value.json_data).unwrap(),
        )
    }
}
