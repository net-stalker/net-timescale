use net_reporter_api::api::network_bandwidth_per_protocol::protocol::ProtocolDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct ProtocolResponse {
    #[sqlx(rename = "Protocol_Name")]
    protocol_name: String,
    #[sqlx(rename = "Total_Bytes")]
    total_bytes: Option<i64>,
}

impl From<ProtocolResponse> for ProtocolDTO {
    fn from(value: ProtocolResponse) -> Self {
        ProtocolDTO::new(
            value.protocol_name.as_str(),
            value.total_bytes.unwrap_or(0),
        )
    }
}