use net_reporter_api::api::total_http_requests::http_requests_bucket::HttpRequestsBucketDTO;
use net_reporter_api::api::total_http_requests::total_http_requests::TotalHttpRequestsDTO;

use super::total_http_requests_bucket::TotalHttpRequestsBucketResponse;

#[derive(Default, Clone, Debug)]
pub struct TotalHttpRequestsResponse {
    total_http_requests_buckets: Vec<TotalHttpRequestsBucketResponse>
}

impl From<TotalHttpRequestsResponse> for TotalHttpRequestsDTO {
    fn from(value: TotalHttpRequestsResponse) -> Self {
        TotalHttpRequestsDTO::new(
            value.total_http_requests_buckets
                .into_iter()
                .map(|total_http_request_bucket | total_http_request_bucket.into())
                .collect::<Vec<HttpRequestsBucketDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<TotalHttpRequestsBucketResponse>> for TotalHttpRequestsResponse {
    fn from(value: Vec<TotalHttpRequestsBucketResponse>) -> Self {
        TotalHttpRequestsResponse {
            total_http_requests_buckets: value
        }
    }
}