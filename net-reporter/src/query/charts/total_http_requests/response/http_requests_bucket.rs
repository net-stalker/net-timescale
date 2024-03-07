use net_reporter_api::api::total_http_requests::http_requests_bucket::HttpRequestsBucketDTO;

use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpRequestsBucketResponse {
    bucket: DateTime<Utc>,
    total_requests: i64,
}

impl From<HttpRequestsBucketResponse> for HttpRequestsBucketDTO {
    fn from(value: HttpRequestsBucketResponse) -> Self {
        HttpRequestsBucketDTO::new(
            value.bucket.timestamp_millis(),
            value.total_requests
        )
    }
}
