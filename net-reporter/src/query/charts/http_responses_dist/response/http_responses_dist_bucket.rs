use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use net_reporter_api::api::http_responses_dist::http_responses_bucket::HttpResponsesBucketDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponsesDistBucketResponse {
    bucket: DateTime<Utc>,
    response_code: i64,
    amount: i64,
}

impl From<HttpResponsesDistBucketResponse> for HttpResponsesBucketDTO {
    fn from(value: HttpResponsesDistBucketResponse) -> Self {
        HttpResponsesBucketDTO::new(
            value.bucket.timestamp_millis(),
            value.response_code,
            value.amount,
        )
    }
}