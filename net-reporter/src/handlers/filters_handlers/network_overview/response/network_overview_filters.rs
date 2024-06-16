use net_reporter_api::api::network::networks::NetworksDTO;
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;

use crate::handlers::network_handlers::networks::response::network::Network;
use crate::handlers::network_handlers::networks::response::networks::Networks;

use super::endpoint_response::EndpointResponse;
use super::protocol_response::ProtocolResponse;

#[derive(Default, Debug)]
pub struct NetworkOverviewFiltersResponse {
    endpoints: Vec<EndpointResponse>,
    protocols: Vec<ProtocolResponse>,
    networks: Vec<Network>,
}

impl NetworkOverviewFiltersResponse {
    pub fn new(
        endpoints: Vec<EndpointResponse>,
        protocols: Vec<ProtocolResponse>,
        networks: Vec<Network>,
    ) -> Self {
        NetworkOverviewFiltersResponse {
            endpoints,
            protocols,
            networks,
        }
    }
}

impl From<NetworkOverviewFiltersResponse> for NetworkOverviewDashboardFiltersDTO {
    fn from(value: NetworkOverviewFiltersResponse) -> Self {
        NetworkOverviewDashboardFiltersDTO::new(
            value.endpoints.into_iter().map(|endpoint| endpoint.endpoint).collect::<Vec<String>>().as_slice(),
            value.protocols.into_iter().map(|protocol| protocol.protocol).collect::<Vec<String>>().as_slice(),
            &NetworksDTO::from(Networks::from(value.networks)),
        )
    }
}
