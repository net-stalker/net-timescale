use std::mem::take;
use std::str::from_utf8;

use jsonpath_rust::{json_path_value, JsonPathFinder};
use serde_json::Value;
use unescape::unescape;

const PATH_SOURCE_LAYER: &str = "$.._source.layers";
const PATH_FRAME_TIME: &str = "$..frame['frame.time']";

pub struct JsonParser;

impl JsonParser {
    fn find(json_binary: Vec<u8>, path: &str) -> Value {
        let json = from_utf8(&json_binary).unwrap();
        let finder = JsonPathFinder::from_str(json, path).expect("path not found");

        finder.find()
    }

    fn print(json_path_value: Value) -> String {
        format!("{}", json_path_value)
    }

    fn pretty(json_path_value: Value) -> String {
        format!("{:#}", json_path_value)
    }

    fn get_string(mut json_path_value: Value) -> String {
        let value = json_path_value.get_mut(0).take()
            .expect("value not found");

        unescape(value.as_str().unwrap()).unwrap()
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

        let json = JsonParser::print(JsonParser::find(pcap_buffer, PATH_SOURCE_LAYER));

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_layer_pretty() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let json = JsonParser::pretty(JsonParser::find(pcap_buffer, PATH_SOURCE_LAYER));

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_frame_time() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let time = JsonParser::get_string(JsonParser::find(pcap_buffer, PATH_FRAME_TIME));

        assert_eq!(time, "Sep 18, 2013 07:49:07.000000000 EEST");
    }

    #[test]
    #[should_panic]
    fn panic_if_unknown_path() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let unknown_path = "";
        JsonParser::find(pcap_buffer, unknown_path);
    }

    #[test]
    #[should_panic]
    fn panic_if_value_not_found() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let unknown_path = "$..frame1";
        let time = JsonParser::get_string(JsonParser::find(pcap_buffer, unknown_path));
    }
}
