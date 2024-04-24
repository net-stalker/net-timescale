use net_reporter_api::api::total_http_requests::http_requests_bucket::HttpRequestsBucketDTO;

use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct TotalHttpRequestsBucketResponse {
    #[sqlx(rename = "Frametime")]
    bucket: DateTime<Utc>,
    #[sqlx(rename = "Total_Requests")]
    total_requests: i64,
}

impl From<TotalHttpRequestsBucketResponse> for HttpRequestsBucketDTO {
    fn from(value: TotalHttpRequestsBucketResponse) -> Self {
        HttpRequestsBucketDTO::new(
            value.bucket.timestamp_millis(),
            value.total_requests
        )
    }
}
