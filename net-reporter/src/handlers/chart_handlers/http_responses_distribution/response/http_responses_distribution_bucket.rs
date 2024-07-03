use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use net_reporter_api::api::http_responses_distribution::http_responses_distribution_bucket::HttpResponsesDistributionBucketDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponsesDistributionBucketResponse {
    #[sqlx(rename = "frametime")]
    bucket: DateTime<Utc>,
    #[sqlx(rename = "response_code")]
    response_code: i64,
    #[sqlx(rename = "amount")]
    amount: i64,
}

impl From<HttpResponsesDistributionBucketResponse> for HttpResponsesDistributionBucketDTO {
    fn from(value: HttpResponsesDistributionBucketResponse) -> Self {
        HttpResponsesDistributionBucketDTO::new(
            value.bucket.timestamp_millis(),
            value.response_code,
            value.amount,
        )
    }
}