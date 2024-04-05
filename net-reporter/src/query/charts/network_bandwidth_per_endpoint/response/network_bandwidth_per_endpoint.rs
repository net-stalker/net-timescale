use net_reporter_api::api::network_bandwidth_per_endpoint::endpoint::EndpointDTO;
use net_reporter_api::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointDTO;

use super::endpoint::EndpointResponse;

#[derive(Default, Clone, Debug)]
pub struct NetworkBandwidthPerEndpointResponse {
    endpoints: Vec<EndpointResponse>
}

impl From<NetworkBandwidthPerEndpointResponse> for NetworkBandwidthPerEndpointDTO {
    fn from(value: NetworkBandwidthPerEndpointResponse) -> Self {
        NetworkBandwidthPerEndpointDTO::new(
            value.endpoints
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<EndpointDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<EndpointResponse>> for NetworkBandwidthPerEndpointResponse {
    fn from(value: Vec<EndpointResponse>) -> Self {
        NetworkBandwidthPerEndpointResponse {
            endpoints: value
        }
    }
}