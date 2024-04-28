use net_file::translator::pcap_translator::PcapTranslator;
use net_file::translator::translator::Translator;

use net_file::jsons::json_parser::JsonParser;
use net_file::jsons::json_pcap_parser::JsonPcapParser;
pub struct Decoder {}

impl Decoder {
    // TODO: change string to a normal error type
    pub async fn get_network_packet_data(pcap_data: &[u8]) -> Result<Vec<u8>, String> {
        let json_bytes = PcapTranslator::translate(pcap_data.to_owned());

        let filtered_value_json = JsonPcapParser::filter_source_layer(&json_bytes);
        let first_json_value = JsonParser::first(&filtered_value_json).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(first_json_value);
        let binary_json = JsonParser::get_vec(layered_json);

        Ok(binary_json)
    }
}