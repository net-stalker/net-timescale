use net_reporter_api::api::network_packet::network_packets::NetworkPacketsDTO;

use super::network_packet::NetworkPacket;

#[derive(Debug)]
pub struct NetworkPackets {
    pub network_packets: Vec<NetworkPacket>,
}

impl From<Vec<NetworkPacket>> for NetworkPackets {
    fn from(value: Vec<NetworkPacket>) -> Self {
        Self {
            network_packets: value
        }
    }
}

impl From<NetworkPackets> for NetworkPacketsDTO {
    fn from(value: NetworkPackets) -> Self {
        NetworkPacketsDTO::new(
            value.network_packets.into_iter().map(|network_packet| network_packet.into()).collect::<Vec<_>>().as_slice(),
        )
    }
}
