use net_reporter_api::api::http_responses::http_responses::HttpResponsesDTO;
use net_reporter_api::api::http_responses::http_response::HttpResponseDTO;


use super::http_response::HttpResponseResponse;

#[derive(Default, Clone, Debug)]
pub struct HttpResponsesResponse {
    http_responses: Vec<HttpResponseResponse>
}

impl From<HttpResponsesResponse> for HttpResponsesDTO {
    fn from(value: HttpResponsesResponse) -> Self {
        HttpResponsesDTO::new(
            value.http_responses
                .into_iter()
                .map(| bandwidth_bucket | bandwidth_bucket.into())
                .collect::<Vec<HttpResponseDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<HttpResponseResponse>> for HttpResponsesResponse {
    fn from(value: Vec<HttpResponseResponse>) -> Self {
        HttpResponsesResponse {
            http_responses: value
        }
    }
}
