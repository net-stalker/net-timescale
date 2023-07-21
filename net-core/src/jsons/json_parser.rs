use std::str::from_utf8;

use chrono::{DateTime, TimeZone, Utc};
use jsonpath_rust::JsonPathFinder;
use serde_json::Value;
use unescape::unescape;

pub struct JsonParser;

impl JsonParser {
    pub fn find(json_binary: &Vec<u8>, path: &str) -> Value {
        let json = from_utf8(&json_binary).unwrap();
        let finder = JsonPathFinder::from_str(json, path).expect("path not found");

        finder.find()
    }
    pub fn first(value: &Value) -> Option<&Value> {
        let value_json = value.as_array().unwrap();
        if value_json.len() > 1 {
            panic!("currently supported only one packet in the file: CU-861mdc6t7")
        }

        value_json.first()
    }

    pub fn print(json_path_value: &Value) -> String {
        format!("{}", json_path_value)
    }

    pub fn pretty(json_path_value: &Value) -> String {
        format!("{:#}", json_path_value)
    }

    pub fn get_vec(json_path_value: Value) -> Vec<u8> {
        serde_json::to_vec(&json_path_value).unwrap()
    }

    pub fn get_string(mut json_path_value: Value) -> String {
        let value = json_path_value.get_mut(0).take()
            .expect("value not found");

        unescape(value.as_str().unwrap()).unwrap()
    }

    pub fn get_utc_frame_time(json_path_value: Value) -> DateTime<Utc> {
        const FORMAT_STR: &str = "%b %d, %Y %H:%M:%S.%f %Z";
        let datetime_str = Self::get_string(json_path_value);

        Utc.datetime_from_str(&*datetime_str, FORMAT_STR).unwrap()
    }

    pub fn get_utc_timestamp_millis(json_path_value: Value) -> i64 {
        let path_value = JsonParser::get_string(json_path_value);
        let timestamp: Vec<&str> = path_value
            .split('.')
            .collect();
        let fraction = &timestamp.get(1).unwrap()[0..3];
        format!("{}{}", &timestamp.get(0).unwrap(), fraction).parse::<i64>().unwrap()
    }

    pub fn get_utc_timestamp_nanos(json_path_value: Value) -> i64 {
        let value_str: String = JsonParser::get_string(json_path_value).split('.').collect();

        value_str.parse::<i64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::file::files::{Files};
    use crate::jsons::json_pcap_parser::PATH_FRAME_TIME_EPOCH;
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_extract_layer() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.json"));
        let json_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted.json"));

        let result = JsonParser::find(&pcap_buffer, "$.._source.layers");
        let json = JsonParser::print(&result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    #[should_panic]
    fn support_only_one_packet_in_file() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/dhcp.pcap"));

        let value = JsonParser::find(&pcap_buffer, "$.._source.layers");
        JsonParser::first(&value);
    }

    #[test]
    fn expected_extract_layer_pretty() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.json"));
        let json_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonParser::find(&pcap_buffer, "$.._source.layers");
        let json = JsonParser::pretty(&result);

        assert_eq!(json, from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_extract_frame_time() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let result = JsonParser::find(&pcap_buffer, "$..frame['frame.time']");
        let time = JsonParser::get_string(result);

        assert_eq!(time, "Sep 18, 2013 04:49:07.000000000 UTC");
    }

    #[test]
    fn expected_get_utc_timestamp_millis() {
        const EXPECTED_TIME: i64 = 1379479747000;

        let pcap_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));
        let value = JsonParser::find(&pcap_buffer, PATH_FRAME_TIME_EPOCH);
        let result = JsonParser::get_utc_timestamp_millis(value);

        assert_eq!(result, EXPECTED_TIME)
    }

    #[test]
    fn expected_get_utc_timestamp_nanos() {
        const EXPECTED_TIME: i64 = 1379479747000000000;

        let pcap_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));
        let value = JsonParser::find(&pcap_buffer, PATH_FRAME_TIME_EPOCH);
        let result = JsonParser::get_utc_timestamp_nanos(value);

        assert_eq!(result, EXPECTED_TIME)
    }

    #[test]
    fn expected_get_utc_timestamp_nanos_2() {
        const EXPECTED_TIME: i64 = 1688714981480935000;

        let pcap_buffer = Files::read_vector(test_resources!("captures/epoch_frame_time.json"));
        let value = JsonParser::find(&pcap_buffer, PATH_FRAME_TIME_EPOCH);
        let result = JsonParser::get_utc_timestamp_nanos(value);

        assert_eq!(result, EXPECTED_TIME)
    }

    #[test]
    #[should_panic]
    fn panic_if_unknown_path() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let unknown_path = "";
        JsonParser::find(&pcap_buffer, unknown_path);
    }

    #[test]
    #[should_panic]
    fn panic_if_value_not_found() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted_pretty.json"));

        let unknown_path = "$..frame1";
        let result = JsonParser::find(&pcap_buffer, unknown_path);
        JsonParser::get_string(result);
    }

    #[test]
    fn expected_extract_layer_and_return_vec() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.json"));
        let json_buffer = Files::read_vector(test_resources!("captures/arp_layer_extracted.json"));

        let result = JsonParser::find(&pcap_buffer, "$.._source.layers");
        let json = JsonParser::get_vec(result);

        assert_eq!(json, json_buffer);
    }

    #[test]
    fn expected_convert_frame_time_to_date_time() {
        let time = Utc.datetime_from_str("Sat, 11 Feb 2023 23:40:00.000000000 EEST", "%a, %d %b %Y %H:%M:%S.%f %Z").unwrap();
        println!("{:?}", time);

        let time = Utc.datetime_from_str("Sep 18, 2013 07:49:07.000000000 EEST", "%b %d, %Y %H:%M:%S.%f %Z").unwrap();
        println!("{:?}", time);

        let time = Utc.datetime_from_str("Dec  5, 2004 21:16:24.317453000 EET", "%b %d, %Y %H:%M:%S.%f %Z").unwrap();
        println!("{:?}", time);

        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.json"));
        let result = JsonParser::find(&pcap_buffer, "$..frame['frame.time']");
        let time = JsonParser::get_utc_frame_time(result);
        println!("{:?}", time);
        assert_eq!(time.to_string(), "2013-09-18 04:49:07 UTC".to_string());
    }
}
