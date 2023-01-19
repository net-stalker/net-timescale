use std::fs::File;
use std::io::Read;
use crate::file::FileReader;

pub struct PCapFile;

impl FileReader for PCapFile {
    fn read(path: &str) -> Vec<u8> {
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_read_pcap_file() {
        let pcap_buffer = PCapFile::read("pcap/test-data.pcap");

        println!("full packet {:?}", pcap_buffer);
        println!("length {}", pcap_buffer.len());

        //24
        let global_header = &pcap_buffer[..24];
        println!("global header {:?}", global_header);

        //16
        let packet_header = &pcap_buffer[24..40];
        println!("packet header {:?}", packet_header);

        //14
        let ethernet_header = &pcap_buffer[40..54];
        println!("ethernet header {:?}", ethernet_header);
    }
}
