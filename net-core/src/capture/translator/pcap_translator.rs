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
    use crate::file::files::Files;
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_translate_arp_packet() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/arp.pcap"));
        let json_result = PcapTranslator::translate(pcap_buffer);

        let json_buffer = r#"[
  {
    "_index": "packets-2013-09-18",
    "_type": "doc",
    "_score": null,
    "_source": {
      "layers": {
        "frame": {
          "frame.encap_type": "26",
          "frame.time": "Sep 18, 2013 07:49:07.000000000 EEST",
          "frame.offset_shift": "0.000000000",
          "frame.time_epoch": "1379479747.000000000",
          "frame.time_delta": "0.000000000",
          "frame.time_delta_displayed": "0.000000000",
          "frame.time_relative": "0.000000000",
          "frame.number": "1",
          "frame.len": "30",
          "frame.cap_len": "30",
          "frame.marked": "0",
          "frame.ignored": "0",
          "frame.protocols": "fr:arp"
        },
        "fr": {
          "fr.first_addr_octet": "0x18",
          "fr.first_addr_octet_tree": {
            "fr.upper_dlci": "0x06",
            "fr.cr": "0",
            "fr.ea": "0"
          },
          "fr.second_addr_octet": "0x41",
          "fr.second_addr_octet_tree": {
            "fr.second_dlci": "0x04",
            "fr.fecn": "0",
            "fr.becn": "0",
            "fr.de": "0",
            "fr.ea": "1"
          },
          "fr.dlci": "100",
          "fr.control": "0x03",
          "fr.control_tree": {
            "fr.control.u_modifier_cmd": "0x00",
            "fr.control.ftype": "0x03"
          },
          "fr.nlpid": [
            "0x00",
            "0x80"
          ],
          "fr.snap.oui": "0",
          "fr.snaptype": "0x0806"
        },
        "arp": {
          "arp.hw.type": "15",
          "arp.proto.type": "0x0800",
          "arp.hw.size": "2",
          "arp.proto.size": "4",
          "arp.opcode": "8",
          "arp.src.hw": "00:00",
          "arp.src.proto_ipv4": "10.206.1.2",
          "arp.dst.hw": "00:64",
          "arp.dst.proto_ipv4": "0.0.0.0"
        }
      }
    }
  }
]
"#.as_bytes();

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }

    #[test]
    #[ignore]
    fn expected_translate_dhcp_packet() {
        let pcap_buffer = Files::read_vector(test_resources!("captures/dhcp.pcap"));
        let json_buffer = Files::read_vector(test_resources!("captures/dhcp.json"));

        let json_result = PcapTranslator::translate(pcap_buffer);

        assert_eq!(std::str::from_utf8(&json_result).unwrap(), std::str::from_utf8(&json_buffer).unwrap());
    }
}
