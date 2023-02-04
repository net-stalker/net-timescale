use serde::{Serialize};
use std::fmt;

// typedef struct pcap_hdr_s {
//     guint32 magic_number;   /* magic number */
//     guint16 version_major;  /* major version number */
//     guint16 version_minor;  /* minor version number */
//     gint32  thiszone;       /* GMT to local correction */
//     guint32 sigfigs;        /* accuracy of timestamps */
//     guint32 snaplen;        /* max length of captured packets, in octets */
//     guint32 network;        /* data link type */
// } pcap_hdr_t;
// Header size = 24 bytes:
#[derive(Serialize)]
pub struct GlobalHeader {
    // magic_number = 4 bytes (d4 c3 b2 a1)
    magic_number: u32,
    // version_major = 2 bytes (02 00)
    version_major: u16,
    // version_minor = 2 bytes (04 00) *in our case 2.4. (little endian)
    version_minor: u16,
    // thiszone = 4 bytes (00 00 00 00) *usually set to 0
    thiszone: u32,
    // sigfigs = 4 bytes (00 00 00 00) *usually set to 0
    sigfigs: u32,
    // snaplen = 4 bytes (FF FF 00 00) *maximum length of the captured packets (data#) in bytes, here its 65535 (0xffff) which is default value for tcpdump and wireshark)
    snaplen: u32,
    // network = 4 bytes (01 00 00 00) *0x1 which indicates that the link-layer protocol is Ethernet. Full list: http://www.tcpdump.org/linktypes.html
    network: u32,
}

impl GlobalHeader {
    pub fn new() -> Self {
        GlobalHeader {
            magic_number: u32::from_be(3569595041),
            version_major: 2,
            version_minor: 4,
            thiszone: 0,
            sigfigs: 0,
            snaplen: 65535,
            network: 1,
        }
    }
}

impl fmt::Debug for GlobalHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GlobalHeader {{ magic_number={}, version_major={}, version_minor={}, thiszone={}, sigfigs={}, snaplen={}, network={}, binary={:?} }}",
               self.magic_number,
               self.version_major,
               self.version_minor,
               self.thiszone,
               self.sigfigs,
               self.snaplen,
               self.network,
               self.to_bytes())
    }
}

impl GlobalHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_create_global_header() {
        let global_header = GlobalHeader::new();
        let buf = global_header.to_bytes();

        println!("{:?}", global_header);
        assert_eq!(24, buf.len());

        assert!([
            212, 195, 178, 161, 2, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0
        ]
            .iter()
            .eq(buf.iter()));
    }
}