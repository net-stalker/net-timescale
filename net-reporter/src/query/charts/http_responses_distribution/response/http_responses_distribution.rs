use net_reporter_api::api::http_responses_distribution::http_responses_distribution_bucket::HttpResponsesDistributionBucketDTO;
use net_reporter_api::api::http_responses_distribution::http_responses_distribution::HttpResponsesDistributionDTO;
use super::http_responses_distribution_bucket::HttpResponsesDistributionBucketResponse;

#[derive(Default, Clone, Debug)]
pub struct HttpResponsesDistributionResponse {
    endpoints: Vec<HttpResponsesDistributionBucketResponse>
}

impl From<HttpResponsesDistributionResponse> for HttpResponsesDistributionDTO {
    fn from(value: HttpResponsesDistributionResponse) -> Self {
        HttpResponsesDistributionDTO::new(
            value.endpoints
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<HttpResponsesDistributionBucketDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<HttpResponsesDistributionBucketResponse>> for HttpResponsesDistributionResponse {
    fn from(value: Vec<HttpResponsesDistributionBucketResponse>) -> Self {
        HttpResponsesDistributionResponse {
            endpoints: value
        }
    }
}