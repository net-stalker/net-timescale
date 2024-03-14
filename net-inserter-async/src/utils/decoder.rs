use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;
pub struct Decoder {}

impl Decoder {
    pub async fn decode(packet: net_agent_api::api::data_packet::DataPacketDTO) -> NetworkPacketDTO {
        todo!()
    }
}