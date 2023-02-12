use std::mem::take;
use std::str::from_utf8;

use jsonpath_rust::{json_path_value, JsonPathFinder};
use serde_json::Value;
use unescape::unescape;

use crate::json_parser::JsonParser;

pub const PATH_SOURCE_LAYER: &str = "$.._source.layers";
pub const PATH_FRAME_TIME: &str = "$..frame['frame.time']";

pub struct JsonPcapParser;

impl JsonPcapParser {
    pub fn find_source_layer(json_binary: Vec<u8>) -> Value {
        JsonParser::find(json_binary, PATH_SOURCE_LAYER)
    }

    pub fn find_frame_time(json_binary: Vec<u8>) -> Value {
        JsonParser::find(json_binary, PATH_FRAME_TIME)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::file::files::{Files, Reader};
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_extract_layer() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted.json"));

        let json = JsonParser::print(JsonPcapParser::find_source_layer(pcap_buffer));

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_layer_pretty() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let json = JsonParser::pretty(JsonPcapParser::find_source_layer(pcap_buffer));

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_frame_time() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let time = JsonParser::get_string(JsonPcapParser::find_frame_time(pcap_buffer));

        assert_eq!(time, "Sep 18, 2013 07:49:07.000000000 EEST");
    }
}
