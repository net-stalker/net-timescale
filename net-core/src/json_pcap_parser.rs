use std::str::from_utf8;

use chrono::{DateTime, Local};
use serde_json::Value;

use crate::json_parser::JsonParser;

pub const PATH_SOURCE_LAYER: &str = "$.._source.layers";
pub const PATH_FRAME_TIME: &str = "$..frame['frame.time']";

pub struct JsonPcapParser;

impl JsonPcapParser {
    pub fn find_source_layer(json_binary: Vec<u8>) -> Value {
        JsonParser::find(json_binary, PATH_SOURCE_LAYER).0
    }

    pub fn find_frame_time(json_binary: Vec<u8>) -> (DateTime<Local>, Vec<u8>) {
        let value = JsonParser::find(json_binary, PATH_FRAME_TIME);

        (JsonParser::get_timestamp_with_tz(value.0), value.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::file::files::{Files, Reader};
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_extract_layer() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted.json"));

        let result = JsonPcapParser::find_source_layer(pcap_buffer);
        let json = JsonParser::print(result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_layer_pretty() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonPcapParser::find_source_layer(pcap_buffer);
        let json = JsonParser::pretty(result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_frame_time() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonPcapParser::find_frame_time(pcap_buffer);

        assert_eq!(result.0.to_string(), "2013-09-18 07:49:07 +03:00");
    }
}
