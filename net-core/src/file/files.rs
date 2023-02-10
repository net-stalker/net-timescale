use std::fs::File;
use std::io::Read;

pub trait Reader {
    fn read(path: &str) -> Vec<u8>;
}

pub struct Files;

impl Reader for Files {
    fn read(path: &str) -> Vec<u8> {
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::test_resources;
    use super::*;

    #[test]
    fn expected_read_file() {
        let buf = Files::read(test_resources!("captures/arp.pcap"));

        println!("full packet {:?}", buf);
        assert_eq!(buf.len(), 70);

        let global_header = &buf[..24];
        println!("global header {:?}", global_header);
        assert_eq!(global_header.len(), 24);

        let packet_header = &buf[24..40];
        println!("packet header {:?}", packet_header);
        assert_eq!(packet_header.len(), 16);

        let ethernet_header = &buf[40..54];
        println!("ethernet header {:?}", ethernet_header);
    }
}
