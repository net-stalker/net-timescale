use std::str::from_utf8;

use chrono::{DateTime, Local};
use serde_json::{json, Value};
use crate::jsons::json_parser::JsonParser;

pub const PATH_SOURCE_LAYER: &str = "$.._source.layers";
pub const PATH_FRAME_TIME: &str = "$..frame['frame.time']";

pub struct JsonPcapParser;

impl JsonPcapParser {
    pub fn filter_source_layer(json_binary: Vec<u8>) -> Value {
        JsonParser::find(json_binary, PATH_SOURCE_LAYER).0
    }

    pub fn find_frame_time(json_binary: Vec<u8>) -> (DateTime<Local>, Vec<u8>) {
        let value = JsonParser::find(json_binary, PATH_FRAME_TIME);

        (JsonParser::get_timestamp_with_tz(value.0), value.1)
    }

    pub fn split_into_layers(value_json: &Value) -> Value {
        let mut new_json = json!({});
        let object_json = new_json.as_object_mut().unwrap();

        value_json.as_object().unwrap()
            .keys()
            .map(|k| k.as_str())
            .enumerate()
            .for_each(|(index, field)| {
                object_json.insert(format!("l{}", index + 1), json!({ field: &value_json[field] }));
            });

        new_json
    }
}

#[cfg(test)]
mod tests {
    use crate::file::files::{Files, Reader};
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_filter_source_layer() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted.json"));

        let result = JsonPcapParser::filter_source_layer(pcap_buffer);
        let json = JsonParser::print(&result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_filter_source_layer_pretty() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonPcapParser::filter_source_layer(pcap_buffer);
        let json = JsonParser::pretty(&result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_frame_time() {
        let pcap_buffer = Files::read(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonPcapParser::find_frame_time(pcap_buffer);

        assert_eq!(result.0.to_string(), "2013-09-18 07:49:07 +03:00");
    }

    #[test]
    fn expected_splited_json_into_layers() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_splied_into_layers.json"));

        let result = JsonParser::find(pcap_buffer, "$.._source.layers");
        let first_value = JsonParser::first(&result.0).unwrap();
        let splited_json = JsonPcapParser::split_into_layers(&first_value);
        let json = JsonParser::pretty(&splited_json);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }
}
