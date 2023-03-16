use std::fs::{File, metadata};
use std::io::Read;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Files;

impl Files {
    pub fn read_string(path: &str) -> String {
        let mut f = File::open(path).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();

        content
    }

    pub fn read_vector(path: &str) -> Vec<u8> {
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        buffer
    }

    pub fn find_files(path_buf: &PathBuf, extension: &str) -> Vec<String> {
        WalkDir::new(path_buf)
            .into_iter()
            .map(|entry| { entry.unwrap() })
            .filter(|entry| { entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == extension) })
            .map(|entry| { entry.path().to_path_buf().to_string_lossy().to_string() })
            .collect()
    }

    pub fn find_rs_files(path_buf: &PathBuf) -> Vec<String> {
        Self::find_files(path_buf, "rs")
    }
}

#[cfg(test)]
mod tests {
    use crate::test_resources;

    use super::*;

    #[test]
    fn expected_read_file_as_binary() {
        let buf = Files::read_vector(test_resources!("captures/arp.pcap"));

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

    #[test]
    fn expected_read_file_string() {
        let content = Files::read_string(test_resources!("captures/arp.json"));

        println!("full packet {:?}", content);
        assert_eq!(content.len(), 1834);
    }
}
