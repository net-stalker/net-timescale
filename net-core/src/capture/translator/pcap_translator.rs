use subprocess::{Exec, Redirection};

use crate::capture::translator::translator::Translator;

pub struct PcapTranslator;

impl Translator for PcapTranslator {
    type Input = Vec<u8>;
    type Output = Vec<u8>;

    /// https://tshark.dev/capture/tshark/
    /// Translate pcap file to json format
    /// # Arguments
    ///
    /// * `buf`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn translate(buf: Vec<u8>) -> Vec<u8> {
        Exec::cmd("tshark")
            .arg("-V") //add output of packet tree        (Packet Details)
            // .arg("-c1") //add output of packet tree        (Packet Details)
            // .arg("-rcaptures/arp.pcap") // set the filename to read from (or '-' for stdin)
            .arg("-r") // set the filename to read from (or '-' for stdin)
            .arg("-")
            // .arg("-x") //add output of hex and ASCII dump (Packet Bytes)
            .arg("-Tjson") //pdml|ps|psml|json|jsonraw|ek|tabs|text|fields| format of text output (def: text)
            .arg("--no-duplicate-keys") // If -T json is specified, merge duplicate keys in an object into a single key with as value a json array containing all values
            .stdin(buf)
            .stdout(Redirection::Pipe)
            .capture().unwrap()
            .stdout
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::file::files::Files;
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_translate_arp_packet() {
        let arp_pcap_path = test_resources!("captures/arp.pcap");
        println!("arp_pcap_path {}", arp_pcap_path);
        println!("found files {:?}", Files::find_files(&PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/test/resources/")), "pcap"));
        let pcap_buffer = Files::read_vector(arp_pcap_path);
        let arp_json_path = test_resources!("captures/arp.json");
        println!("arp_json_path {}", arp_json_path);
        let json_buffer = Files::read_vector(arp_json_path);

        let json_result = PcapTranslator::translate(pcap_buffer);

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }

    #[test]
    fn expected_translate_dhcp_packet() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/dhcp.pcap"));
        let json_buffer = Files::read_vector(test_resources!("captures/dhcp.json"));

        let json_result = PcapTranslator::translate(pcap_buffer);

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }
}
