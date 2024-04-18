use chrono::DateTime;
use chrono::Utc;
use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;
use sqlx::Error;
use sqlx::Postgres;

const INSERT_NP_QUERY: &str = "
    INSERT INTO Traffic (Insertion_Time, Tenant_Id, Raw_Pcap_File_Path, Parsed_Data) VALUES (NOW(), $1, $2, $3)
    RETURNING
        Pcap_ID as id,
        Insertion_Time as frame_time,
        Parsed_Data->'l3'->'ip'->>'ip.src' AS src,
        Parsed_Data->'l3'->'ip'->>'ip.dst' AS dst,
        string_to_array(Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') as protocols,
        Parsed_Data as json_data
";


#[derive(sqlx::FromRow)]
struct NetworkPacket {
    pub id: i32,
    pub frame_time: DateTime<Utc>,
    pub src: String,
    pub dst: String,
    pub protocols: Vec<String>,
    // we skip network id because it is null in fresh inserter network packet
    pub json_data: serde_json::Value,
}

impl From<NetworkPacket> for NetworkPacketDTO {
    fn from(value: NetworkPacket) -> Self {
        NetworkPacketDTO::new(
            value.id as i64,
            value.frame_time.timestamp_nanos_opt().unwrap_or_default(),
            &value.src,
            &value.dst,
            &value.protocols,
            &serde_json::to_vec(&value.json_data).unwrap_or_default(),
        )
    }
}

pub async fn insert_network_packet_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    tenant_id: &str,
    pcap_file_path: &str,
    network_packet_data: &[u8],
) -> Result<NetworkPacketDTO, Error> {
    let binary_data: serde_json::Value = match serde_json::from_slice(network_packet_data) {
        Ok(data) => data,
        Err(_) => {
            log::error!("Failed to decode network packet data");
            return Err(Error::Decode(Box::new(sqlx::error::Error::Protocol(
                "Failed to decode network packet data".to_string()
            ))))
        }
    };
    let network_packet = sqlx::query_as::<_, NetworkPacket>(INSERT_NP_QUERY)
        .bind(tenant_id)
        .bind(pcap_file_path)
        .bind(binary_data)
        .fetch_one(&mut **transaction)
        .await?;
    Ok(network_packet.into())
}
