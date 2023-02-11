use std::str::from_utf8;
use jsonpath_rust::JsonPathFinder;
use mockall::PredicateStrExt;
use subprocess::{Exec, Redirection};

use crate::translator::Decoder;

const PATH: &str = "$.._source.layers";

pub struct JsonExtractor;

impl Decoder for JsonExtractor {
    type Input = Vec<u8>;
    type Output = String;

    fn decode(json_binary: Vec<u8>) -> String {
        let json = from_utf8(&json_binary).unwrap();
        let finder = JsonPathFinder::from_str(json, PATH).unwrap();
        let value = finder.find();

        format!("{:#}", value)
    }
}

#[cfg(test)]
mod tests {
    use crate::capture::decoder_binary::JsonDecoder;
    use crate::test_resources;
    use crate::file::files::{Files, Reader};

    use super::*;

    #[test]
    fn expected_extract_layer() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.pcap"));
        let json_buffer = Files::read(test_resources!("captures/arp_sliced.json"));

        let json_result = JsonDecoder::decode(pcap_buffer);
        let string = JsonExtractor::decode(json_result);

        assert_eq!(string, from_utf8(&json_buffer).unwrap());
    }
}
