use net_reporter_api::api::network_bandwidth_per_endpoint::endpoint::EndpointDTO;


#[derive(sqlx::FromRow, Clone, Debug)]
pub struct EndpointResponse {
    id: String,
    bytes_sent: Option<i64>,
    bytes_received: Option<i64>,
}

impl From<EndpointResponse> for EndpointDTO {
    fn from(value: EndpointResponse) -> Self {
        EndpointDTO::new(
            value.id.as_str(),
            value.bytes_received.unwrap_or(0),
            value.bytes_sent.unwrap_or(0),
        )
    }
}