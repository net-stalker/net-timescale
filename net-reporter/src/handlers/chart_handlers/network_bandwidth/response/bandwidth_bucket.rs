use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use net_reporter_api::api::network_bandwidth::bandwidth_bucket::BandwidthBucketDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct BandwidthBucketResponse {
    #[sqlx(rename = "frametime")]
    bucket: DateTime<Utc>,
    #[sqlx(rename = "total_bytes")]
    total_bytes: i64,
}

impl From<BandwidthBucketResponse> for BandwidthBucketDTO {
    fn from(value: BandwidthBucketResponse) -> Self {
        BandwidthBucketDTO::new(
            value.bucket.timestamp_millis(),
            value.total_bytes
        )
    }
}