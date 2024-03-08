use net_reporter_api::api::http_responses_dist::{http_responses_bucket::HttpResponsesBucketDTO, http_responses_dist::HttpResponsesDistDTO};

use super::http_responses_dist_bucket::HttpResponsesDistBucketResponse;

#[derive(Default, Clone, Debug)]
pub struct HttpResponsesDistResponse {
    endpoints: Vec<HttpResponsesDistBucketResponse>
}

impl From<HttpResponsesDistResponse> for HttpResponsesDistDTO {
    fn from(value: HttpResponsesDistResponse) -> Self {
        HttpResponsesDistDTO::new(
            value.endpoints
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<HttpResponsesBucketDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<HttpResponsesDistBucketResponse>> for HttpResponsesDistResponse {
    fn from(value: Vec<HttpResponsesDistBucketResponse>) -> Self {
        HttpResponsesDistResponse {
            endpoints: value
        }
    }
}