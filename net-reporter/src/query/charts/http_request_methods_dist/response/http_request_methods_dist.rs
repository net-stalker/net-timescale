use net_reporter_api::api::http_request_methods_dist::http_request::HttpRequestDTO;
use net_reporter_api::api::http_request_methods_dist::http_request_methods_dist::HttpRequestMethodsDistDTO;

use super::http_request::HttpRequestResponse;

#[derive(Default, Clone, Debug)]
pub struct HttpRequestMethodsDistResponse {
    http_requests: Vec<HttpRequestResponse>
}

impl From<HttpRequestMethodsDistResponse> for HttpRequestMethodsDistDTO {
    fn from(value: HttpRequestMethodsDistResponse) -> Self {
        HttpRequestMethodsDistDTO::new(
            value.http_requests
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<HttpRequestDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<HttpRequestResponse>> for HttpRequestMethodsDistResponse {
    fn from(value: Vec<HttpRequestResponse>) -> Self {
        HttpRequestMethodsDistResponse {
            http_requests: value
        }
    }
}