use crate::translator::Decoder;
use subprocess::{Exec, Redirection};

pub struct JsonDecoder;

impl Decoder for JsonDecoder {
    /// https://tshark.dev/capture/tshark/
    ///
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
    fn decode(buf: Vec<u8>) -> String {
        return Exec::cmd("tshark")
            .arg("-V") //add output of packet tree        (Packet Details)
            // .arg("-c1") //add output of packet tree        (Packet Details)
            // .arg("-rcaptures/arp.pcap") // set the filename to read from (or '-' for stdin)
            .arg("-r") // set the filename to read from (or '-' for stdin)
            .arg("-")
            // .arg("-x") //add output of hex and ASCII dump (Packet Bytes)
            .arg("-Tjson") //pdml|ps|psml|json|jsonraw|ek|tabs|text|fields| format of text output (def: text)
            .stdin(buf)
            .stdout(Redirection::Pipe)
            .capture()
            .unwrap()
            .stdout_str();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::pcap_file::PCapFile;
    use crate::file::Reader;

    #[test]
    fn expected_decode_pcap() {
        let pcap_buffer = PCapFile::read("../net-core/captures/arp.pcap");
        let json_buffer = PCapFile::read("../net-core/captures/arp.json");

        let json_result = JsonDecoder::decode(pcap_buffer);

        assert_eq!(json_result, std::str::from_utf8(&json_buffer).unwrap());
    }
}
