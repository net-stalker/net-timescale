use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use unescape::unescape;

use crate::jsons::json_parser::JsonParser;

pub const PATH_SOURCE_LAYER: &str = "$.._source.layers";
pub const PATH_FRAME_TIME: &str = "$..frame['frame.time']";
const L3_PATH: &'static str = "/l3";

pub struct JsonPcapParser;

impl JsonPcapParser {
    pub fn filter_source_layer(json_binary: &Vec<u8>) -> Value {
        JsonParser::find(json_binary, PATH_SOURCE_LAYER)
    }

    pub fn find_frame_time(json_binary: &Vec<u8>) -> DateTime<Utc> {
        let value = JsonParser::find(json_binary, PATH_FRAME_TIME);
        // this stuff is returning DateTime<Local>, though in binary we have Utc timestamp format 
        JsonParser::get_timestamp_with_tz(value)
    }

    pub fn split_into_layers(value_json: &Value) -> Value {
        let mut new_json = json!({});
        let object_json = new_json.as_object_mut().unwrap();

        value_json
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.as_str())
            .enumerate()
            .for_each(|(index, field)| {
                object_json.insert(
                    format!("l{}", index + 1),
                    json!({ field: &value_json[field] }),
                );
            });

        new_json
    }

    fn extract_field_name(l3_value: &Value) -> &str {
        l3_value
            .as_object()
            .unwrap()
            .keys()
            .map(|k| k.as_str())
            .last()
            .unwrap()
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `l3_field_name`:
    ///
    /// returns: &str
    ///
    /// # Examples
    ///
    /// ```json
    /// {
    ///     "ip": {
    //          "ip.version": "4",
    //          "ip.src": "0.0.0.0",
    //          "ip.hdr_len": "20",
    //          "ip.dsfield": "0x00"
    //      }
    //  }
    //
    // /ip/ip.src
    // /ip/ip.dst
    //
    /// ```
    fn create_src_addr_path(field_name_prefix: &str, field_name_suffix: &str) -> String {
        format!(
            "/{}/{}.{}",
            field_name_prefix, field_name_prefix, field_name_suffix
        )
    }

    fn extract_ip_addr_l3(json_value: &Value, target: &str) -> Option<String> {
        let l3_value = json_value.pointer(L3_PATH).unwrap();
        let l3_field_name = Self::extract_field_name(l3_value);
        let addr_path = Self::create_src_addr_path(l3_field_name, target);
        let addr_value = l3_value.pointer(addr_path.as_str());

        match addr_value {
            None => None,
            Some(addr_value) => unescape(addr_value.as_str().unwrap()),
        }
    }

    pub fn extract_src_addr_l3(json_value: &Value) -> Option<String> {
        Self::extract_ip_addr_l3(json_value, "src")
    }

    pub fn extract_dst_addr_l3(json_value: &Value) -> Option<String> {
        Self::extract_ip_addr_l3(json_value, "dst")
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use crate::file::files::Files;
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_filter_source_layer() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.json"));
        let json_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted.json"));

        let result = JsonPcapParser::filter_source_layer(&pcap_buffer);
        let json = JsonParser::print(&result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_filter_source_layer_pretty() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.json"));
        let json_buffer =
            Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonPcapParser::filter_source_layer(&pcap_buffer);
        let json = JsonParser::pretty(&result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_frame_time() {
        let pcap_buffer =
            Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonPcapParser::find_frame_time(&pcap_buffer);

        assert_eq!(result.to_string(), "2013-09-18 04:49:07 UTC");
    }

    #[test]
    fn expected_layered_json_into_layers() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.json"));
        let json_buffer = Files::read_vector(test_resources!("captures/arp_layers.json"));

        let result = JsonParser::find(&pcap_buffer, "$.._source.layers");
        let first_value = JsonParser::first(&result).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(&first_value);
        let json = JsonParser::pretty(&layered_json);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_src_address_from_l3() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/dhcp_one_packet.json"));

        let result = JsonParser::find(&pcap_buffer, "$.._source.layers");
        let first_value = JsonParser::first(&result).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(&first_value);

        let string = JsonPcapParser::extract_src_addr_l3(&layered_json).unwrap();

        assert_eq!(string, "0.0.0.0");
    }

    #[test]
    fn expected_extract_dst_address_from_l3() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/dhcp_one_packet.json"));

        let result = JsonParser::find(&pcap_buffer, "$.._source.layers");
        let first_value = JsonParser::first(&result).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(&first_value);

        let string = JsonPcapParser::extract_dst_addr_l3(&layered_json).unwrap();

        assert_eq!(string, "255.255.255.255");
    }

    #[test]
    fn expected_none_when_path_invalid() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/dhcp_one_packet.json"));

        let result = JsonParser::find(&pcap_buffer, "$.._source.layers");
        let first_value = JsonParser::first(&result).unwrap();
        let layered_json = JsonPcapParser::split_into_layers(&first_value);

        let string = JsonPcapParser::extract_ip_addr_l3(&layered_json, "any");

        assert_eq!(string, None);
    }
}
