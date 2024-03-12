use net_reporter_api::api::http_clients::http_client::HttpClientDTO;
use net_reporter_api::api::http_clients::http_clients::HttpClientsDTO;

use super::http_client::HttpClientResponse;

#[derive(Default, Clone, Debug)]
pub struct HttpClientsResponse {
    http_clients: Vec<HttpClientResponse>
}

impl From<HttpClientsResponse> for HttpClientsDTO {
    fn from(value: HttpClientsResponse) -> Self {
        HttpClientsDTO::new(
            value.http_clients
                .into_iter()
                .map(| bandwidth_bucket | bandwidth_bucket.into())
                .collect::<Vec<HttpClientDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<HttpClientResponse>> for HttpClientsResponse {
    fn from(value: Vec<HttpClientResponse>) -> Self {
        HttpClientsResponse {
            http_clients: value
        }
    }
}