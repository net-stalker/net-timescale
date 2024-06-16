use net_reporter_api::api::http_overview_dashboard_filters::http_overview_dashboard_filters::HttpOverviewDashboardFiltersDTO;
use net_reporter_api::api::network::networks::NetworksDTO;

use crate::handlers::network_handlers::networks::response::network::Network;
use crate::handlers::network_handlers::networks::response::networks::Networks;

use super::endpoint_response::EndpointResponse;
use super::http_request_method_response::HttpRequestMethodResponse;
use super::http_response_code_response::HttpResponseCodeResponse;


#[derive(Default, Clone, Debug)]
pub struct HttpOverviewFiltersResponse {
    endpoints: Vec<EndpointResponse>,
    http_request_methods: Vec<HttpRequestMethodResponse>,
    http_response_codes: Vec<HttpResponseCodeResponse>,
    networks: Vec<Network>,
}

impl HttpOverviewFiltersResponse {
    pub fn new(
        endpoints: Vec<EndpointResponse>,
        http_request_methods: Vec<HttpRequestMethodResponse>,
        http_response_codes: Vec<HttpResponseCodeResponse>,
        networks: Vec<Network>,
    ) -> Self {
        HttpOverviewFiltersResponse {
            endpoints,
            http_request_methods,
            http_response_codes,
            networks,
        }
    }
}

impl From<HttpOverviewFiltersResponse> for HttpOverviewDashboardFiltersDTO {
    fn from(value: HttpOverviewFiltersResponse) -> Self {
        HttpOverviewDashboardFiltersDTO::new(
            value.endpoints.into_iter().map(|endpoint| endpoint.endpoint).collect::<Vec<String>>().as_slice(),
            value.http_request_methods.into_iter().map(|method| method.http_request_method).collect::<Vec<String>>().as_slice(),
            value.http_response_codes.into_iter().map(|code| code.http_response_code).collect::<Vec<String>>().as_slice(),
            &NetworksDTO::from(Networks::from(value.networks)),
        )
    }
}
