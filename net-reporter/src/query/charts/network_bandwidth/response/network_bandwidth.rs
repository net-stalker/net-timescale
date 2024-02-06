use net_timescale_api::api::network_bandwidth::bandwidth_bucket::BandwidthBucketDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;

use super::bandwidth_bucket::BandwidthBucketResponse;

#[derive(Default, Clone, Debug)]
pub struct NetworkBandwidthResponse {
    bandwidth_buckets: Vec<BandwidthBucketResponse>
}

impl From<NetworkBandwidthResponse> for NetworkBandwidthDTO {
    fn from(value: NetworkBandwidthResponse) -> Self {
        NetworkBandwidthDTO::new(
            value.bandwidth_buckets
                .into_iter()
                .map(| bandwidth_bucket | bandwidth_bucket.into())
                .collect::<Vec<BandwidthBucketDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<BandwidthBucketResponse>> for NetworkBandwidthResponse {
    fn from(value: Vec<BandwidthBucketResponse>) -> Self {
        NetworkBandwidthResponse {
            bandwidth_buckets: value
        }
    }
}