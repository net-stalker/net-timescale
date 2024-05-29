use net_reporter_api::api::network::networks::NetworksDTO;

use super::network::Network;

#[derive(Debug)]
pub struct Networks {
    pub networks: Vec<Network>,
}

impl From<Vec<Network>> for Networks {
    fn from(value: Vec<Network>) -> Self {
        Self { networks: value }
    }
}

impl From<Networks> for NetworksDTO {
    fn from(value: Networks) -> Self {
        NetworksDTO::new(
            value.networks.into_iter().map(|network| network.into()).collect::<Vec<_>>().as_slice()
        )
    }
}