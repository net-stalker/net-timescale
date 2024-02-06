use net_timescale_api::api::bandwidth_per_endpoint::bandwidth_per_endpoint::BandwidthPerEndpointDTO;
use net_timescale_api::api::bandwidth_per_endpoint::endpoint::EndpointDTO;

use super::endpoint::EndpointResponse;

#[derive(Default, Clone, Debug)]
pub struct BandwidthPerEndpointResponse {
    endpoints: Vec<EndpointResponse>
}

impl From<BandwidthPerEndpointResponse> for BandwidthPerEndpointDTO {
    fn from(value: BandwidthPerEndpointResponse) -> Self {
        BandwidthPerEndpointDTO::new(
            value.endpoints
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<EndpointDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<EndpointResponse>> for BandwidthPerEndpointResponse {
    fn from(value: Vec<EndpointResponse>) -> Self {
        BandwidthPerEndpointResponse {
            endpoints: value
        }
    }
}