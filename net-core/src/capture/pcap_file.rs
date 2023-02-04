use std::fs::File;
use std::io::Read;
use crate::file::Reader;

pub struct PCapFile;

impl Reader for PCapFile {
    fn read(path: &str) -> Vec<u8> {
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        buffer
    }
}

#[cfg(test)]
mod tests {
    use pcap::{Capture, Packet};
    use crate::capture::polling::{Handler, Poller};
    use super::*;

    #[test]
    fn expected_read_pcap_file() {
        let pcap_buffer = PCapFile::read("../net-core/captures/arp.pcap");

        println!("full packet {:?}", pcap_buffer);
        assert_eq!(pcap_buffer.len(), 70);

        let global_header = &pcap_buffer[..24];
        println!("global header {:?}", global_header);
        assert_eq!(global_header.len(), 24);

        let packet_header = &pcap_buffer[24..40];
        println!("packet header {:?}", packet_header);
        assert_eq!(packet_header.len(), 16);

        let ethernet_header = &pcap_buffer[40..54];
        println!("ethernet header {:?}", ethernet_header);
    }
}
