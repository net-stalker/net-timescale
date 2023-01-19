// https://tshark.dev/capture/tshark/

use subprocess::{Exec, Redirection};
use crate::translator::Decoder;

pub struct BinaryDecoder;

impl Decoder for BinaryDecoder {
    fn decode(buf: Vec<u8>) -> String {
        return Exec::cmd("tshark")
            .arg("-V") //add output of packet tree        (Packet Details)
            // .arg("-rcaptures/arp.pcap") // set the filename to read from (or '-' for stdin)
            .arg("-r") // set the filename to read from (or '-' for stdin)
            .arg("-")
            // .arg("-x") //add output of hex and ASCII dump (Packet Bytes)
            .arg("-Tjson") //pdml|ps|psml|json|jsonraw|ek|tabs|text|fields| format of text output (def: text)
            .stdin(buf)
            .stdout(Redirection::Pipe)
            .capture()
            .unwrap()
            .stdout_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::capture::pcap_file::PCapFile;
    use crate::file::FileReader;

    use super::*;

    #[test]
    fn subprocess() {
        let pcap_buffer = PCapFile::read("../net-core/captures/arp.pcap");

        let out = BinaryDecoder::decode(pcap_buffer);
        println!("{}", out);
        // assert_eq!(out, "");
    }
}
