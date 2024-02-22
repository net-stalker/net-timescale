use net_reporter_api::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolDTO;
use net_reporter_api::api::network_bandwidth_per_protocol::protocol::ProtocolDTO;

use super::protocol::ProtocolResponse;

#[derive(Default, Clone, Debug)]
pub struct NetworkBandwidthPerProtocolResponse {
    endpoints: Vec<ProtocolResponse>
}

impl From<NetworkBandwidthPerProtocolResponse> for NetworkBandwidthPerProtocolDTO {
    fn from(value: NetworkBandwidthPerProtocolResponse) -> Self {
        NetworkBandwidthPerProtocolDTO::new(
            value.endpoints
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<ProtocolDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<ProtocolResponse>> for NetworkBandwidthPerProtocolResponse {
    fn from(value: Vec<ProtocolResponse>) -> Self {
        NetworkBandwidthPerProtocolResponse {
            endpoints: value
        }
    }
}