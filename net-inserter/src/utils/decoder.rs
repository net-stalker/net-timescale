use net_inserter_api::api::network_packet::network_packet::NetworkPacketDTO;

use net_file::translator::pcap_translator::PcapTranslator;
use net_file::translator::translator::Translator;

use net_file::jsons::json_parser::JsonParser;
use net_file::jsons::json_pcap_parser::JsonPcapParser;
pub struct Decoder {}

impl Decoder {
    // TODO: change string to a normal error type
    pub async fn decode(pcap_data: &[u8]) -> Result<NetworkPacketDTO, String> {
        let json_bytes = PcapTranslator::translate(pcap_data.to_owned());

        let filtered_value_json = JsonPcapParser::filter_source_layer(&json_bytes);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);
        let frame_time = JsonPcapParser::find_utc_timestamp_nanos(&json_bytes);
        let src_addr = match JsonPcapParser::extract_src_addr_l3(&layered_json) {
            Some(src) => src,
            None => {
                log::error!("src is missing");
                return Err("src is missing".to_string());
            }
        };
        let dst_addr = match JsonPcapParser::extract_dst_addr_l3(&layered_json) {
            Some(dst) => dst,
            None => {
                log::error!("dst is missing");
                return Err("dst is missing".to_string());
            }
        };
        let binary_json = JsonParser::get_vec(layered_json);

        Ok(NetworkPacketDTO::new(
            frame_time,
            &src_addr,
            &dst_addr,
            &binary_json
        ))
        
    }
}