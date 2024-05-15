use net_reporter_api::api::http_request_methods_distribution::http_request_method::HttpRequestMethodDTO;
use net_reporter_api::api::http_request_methods_distribution::http_request_methods_distribution::HttpRequestMethodsDistributionDTO;

use super::http_request::HttpRequestMethodResponse;

#[derive(Default, Clone, Debug)]
pub struct HttpRequestMethodsDistributionResponse {
    http_requests: Vec<HttpRequestMethodResponse>
}

impl From<HttpRequestMethodsDistributionResponse> for HttpRequestMethodsDistributionDTO {
    fn from(value: HttpRequestMethodsDistributionResponse) -> Self {
        HttpRequestMethodsDistributionDTO::new(
            value.http_requests
                .into_iter()
                .map(|endpoint| endpoint.into())
                .collect::<Vec<HttpRequestMethodDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<HttpRequestMethodResponse>> for HttpRequestMethodsDistributionResponse {
    fn from(value: Vec<HttpRequestMethodResponse>) -> Self {
        HttpRequestMethodsDistributionResponse {
            http_requests: value
        }
    }
}