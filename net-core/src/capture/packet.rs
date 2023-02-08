use serde::{Serialize};
use std::fmt;
use std::ops::Deref;

// https://tshark.dev/formats/pcap_deconstruction/
// Packet header
// typedef struct pcaprec_hdr_s {
//     guint32 ts_sec;         /* timestamp seconds */
//     guint32 ts_usec;        /* timestamp microseconds */
//     guint32 incl_len;       /* number of octets of packet saved in file */
//     guint32 orig_len;       /* actual length of packet */
// } pcaprec_hdr_t;
// Packet header size = 16 bytes
#[derive(Serialize)]
pub struct Packet {
    // ts_sec = 4 bytes (85 AD C7 50) *This is the number of seconds since the start of 1970, also known as Unix Epoch
    tv_sec: u32,
    // ts_usec = 4 bytes (AC 97 05 00) *microseconds part of the time at which the packet was captured
    tv_usec: u32,
    // The number of bytes of the packet that are available from the capture
    // incl_len = 4 bytes (E0 04 00 00) = 1248 *contains the size of the saved packet data in our file in bytes (following the header)
    caplen: u32,
    // The length of the packet, in bytes (which might be more than the number of bytes available
    // from the capture, if the length of the packet is larger than the maximum number of bytes to
    // capture)
    // orig_len = 4 bytes (E0 04 00 00) *Both fields' value is same here, but these may have different values in cases where we set the maximum packet length (whose value is 65535 in the global header of our file) to a smaller size.
    len: u32,
    #[serde(skip_serializing)]
    data: Vec<u8>,
}

impl From<pcap::Packet<'_>> for Packet {
    fn from(pcap_paket: pcap::Packet) -> Self {
        Packet {
            tv_sec: pcap_paket.header.ts.tv_sec as u32,
            tv_usec: pcap_paket.header.ts.tv_usec as u32,
            caplen: pcap_paket.header.caplen as u32,
            len: pcap_paket.header.len as u32,
            data: pcap_paket.data.deref().to_vec(),
        }
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Packet {{ ts: {}.{:06}, caplen: {}, len: {}, data: {:?}, binary: {:?} }}",
            self.tv_sec, self.tv_usec, self.caplen, self.len, self.data, self.to_bytes()
        )
    }
}


impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = bincode::serialize(&self).unwrap();
        bytes.append(&mut self.data.deref().to_vec());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use pcap::Capture;
    use super::*;

    #[test]
    fn expected_encode_file() {
        let mut capture = Capture::from_file("../net-core/captures/arp.pcap").unwrap();
        let packet = capture.next_packet().unwrap();

        let packet = Packet::from(packet);
        println!("{:?}", packet);

        assert_eq!(46, packet.to_bytes().len());
    }
}