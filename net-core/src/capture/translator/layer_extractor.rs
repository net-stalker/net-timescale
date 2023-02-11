use std::str::from_utf8;
use jsonpath_rust::JsonPathFinder;
use mockall::PredicateStrExt;
use crate::capture::translator::translator::Translator;

const PATH: &str = "$.._source.layers";

pub struct LayerExtractor;

impl Translator for LayerExtractor {
    type Input = Vec<u8>;
    type Output = String;

    fn translate(json_binary: Vec<u8>) -> String {
        let json = from_utf8(&json_binary).unwrap();
        let finder = JsonPathFinder::from_str(json, PATH).unwrap();
        let value = finder.find();

        format!("{:#}", value)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_resources;
    use crate::file::files::{Files, Reader};

    use super::*;

    #[test]
    fn expected_extract_layer() {
        let pcap_buffer = Files::read(test_resources!("captures/arp.json"));
        let json_buffer = Files::read(test_resources!("captures/arp_sliced.json"));

        let string = LayerExtractor::translate(pcap_buffer);

        assert_eq!(string, from_utf8(&json_buffer).unwrap());
    }
}
