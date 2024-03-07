use net_reporter_api::api::total_http_requests::http_requests_bucket::HttpRequestsBucketDTO;
use net_reporter_api::api::total_http_requests::total_http_requests::TotalHttpRequestsDTO;

use super::http_requests_bucket::HttpRequestsBucketResponse;

#[derive(Default, Clone, Debug)]
pub struct TotalHttpRequestsResponse {
    http_requests_buckets: Vec<HttpRequestsBucketResponse>
}

impl From<TotalHttpRequestsResponse> for TotalHttpRequestsDTO {
    fn from(value: TotalHttpRequestsResponse) -> Self {
        TotalHttpRequestsDTO::new(
            value.http_requests_buckets
                .into_iter()
                .map(| bandwidth_bucket | bandwidth_bucket.into())
                .collect::<Vec<HttpRequestsBucketDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<HttpRequestsBucketResponse>> for TotalHttpRequestsResponse {
    fn from(value: Vec<HttpRequestsBucketResponse>) -> Self {
        TotalHttpRequestsResponse {
            http_requests_buckets: value
        }
    }
}