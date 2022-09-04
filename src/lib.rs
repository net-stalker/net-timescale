use serde::{Deserialize, Serialize};
use std::fmt;

use pcap::{Capture, Device};

pub fn print_devices() {
    Device::list().unwrap().iter().for_each(|device| {
        println!(
            "Device: {}, {:?}, {:?}",
            device.name,
            device.desc.as_ref(),
            device.addresses
        );
    });
}

pub fn lookup_default_device() -> Device {
    let default_devide = Device::lookup().unwrap();
    match default_devide {
        Some(devide) => {
            println!(
                "Default Device: {}, {:?}, {:?}",
                devide.name,
                devide.desc.as_ref(),
                devide.addresses
            );

            devide
        }
        None => todo!(),
    }
}

pub fn create_global_header() -> GlobalHeader {
    // let mut rng = rand::thread_rng();
    // let n1: u8 = rng.gen();

    GlobalHeader {
        magic_number: u32::from_be(3569595041),
        version_major: u16::from_le(2),
        version_minor: u16::from_le(4),
        thiszone: 0,
        sigfigs: 0,
        snaplen: u32::from_le(65535),
        network: u32::from_le(1),
    }
}

pub fn capture_packages(cnt_packages: i16, f: impl Fn(i16, PcapPacket)) {
    let mut cap = Capture::from_device(lookup_default_device())
        .unwrap()
        // .promisc(true)
        // .snaplen(65535)
        .buffer_size(1000)
        .open()
        .unwrap();

    let mut cnt = 0;
    while let Ok(packet) = cap.next_packet() {
        if cnt_packages == cnt {
            break;
        }
        cnt += 1;

        let packet = PcapPacket {
            tv_sec: packet.header.ts.tv_sec as u32,
            tv_usec: packet.header.ts.tv_usec as u32,
            caplen: packet.header.caplen,
            len: packet.header.len,
            data: packet.data,
        };
        println!("Received packet: cnt={} packet={}", cnt, packet);

        f(cnt, packet);
    }
}

// GLOBAL HEADER
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

// magic_number = 4 bytes (d4 c3 b2 a1)
// version_major = 2 bytes (02 00)
// version_minor = 2 bytes (04 00) *in our case 2.4. (little endian)
// thiszone = 4 bytes (00 00 00 00) *usually set to 0
// sigfigs = 4 bytes (00 00 00 00) *usually set to 0
// snaplen = 4 bytes (FF FF 00 00) *maximum length of the captured packets (data#) in bytes, here its 65535 (0xffff) which is default value for tcpdump and wireshark)
// network = 4 bytes (01 00 00 00) *0x1 which indicates that the link-layer protocol is Ethernet. Full list: http://www.tcpdump.org/linktypes.html
#[derive(Serialize, Deserialize)]
pub struct GlobalHeader {
    magic_number: u32,
    version_major: u16,
    version_minor: u16,
    thiszone: u32,
    sigfigs: u32,
    snaplen: u32,
    network: u32,
}

impl fmt::Display for GlobalHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(
            magic_number={}, 
            version_major={}, 
            version_minor={}, 
            thiszone={}, 
            sigfigs={}, 
            snaplen={}, 
            network={}
        )",
            self.magic_number,
            self.version_major,
            self.version_minor,
            self.thiszone,
            self.sigfigs,
            self.snaplen,
            self.network
        )
    }
}

impl GlobalHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
        let global_header_bytes = bincode::serialize(&self).unwrap();
        println!(
            "Length={} Global header: {:?} ",
            global_header_bytes.len(),
            global_header_bytes
        );

        global_header_bytes
    }
}

// Packet header
// typedef struct pcaprec_hdr_s {
//     guint32 ts_sec;         /* timestamp seconds */
//     guint32 ts_usec;        /* timestamp microseconds */
//     guint32 incl_len;       /* number of octets of packet saved in file */
//     guint32 orig_len;       /* actual length of packet */
// } pcaprec_hdr_t;

// Packet header size = 16 bytes

// ts_sec = 4 bytes (85 AD C7 50) *This is the number of seconds since the start of 1970, also known as Unix Epoch
// ts_usec = 4 bytes (AC 97 05 00) *microseconds part of the time at which the packet was captured
// incl_len = 4 bytes (E0 04 00 00) = 1248 *contains the size of the saved packet data in our file in bytes (following the header)
// orig_len = 4 bytes (E0 04 00 00) *Both fields' value is same here, but these may have different values in cases where we set the maximum packet length (whose value is 65535 in the global header of our file) to a smaller size.
#[derive(Serialize, Deserialize, Debug)]
pub struct PcapPacket<'a> {
    /// The time when the packet was captured
    pub tv_sec: u32,
    pub tv_usec: u32,
    /// The number of bytes of the packet that are available from the capture
    pub caplen: u32,
    /// The length of the packet, in bytes (which might be more than the number of bytes available
    /// from the capture, if the length of the packet is larger than the maximum number of bytes to
    /// capture)
    pub len: u32,
    pub data: &'a [u8],
}

impl<'a> fmt::Display for PcapPacket<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(tv_sec={} tv_usec={} caplen={} len={} data={:?})",
            self.tv_sec, self.tv_usec, self.caplen, self.len, self.data
        )
    }
}

impl<'a> PcapPacket<'a> {
    pub fn as_bytes(&self) -> Vec<u8> {
        let packet_header_bytes = bincode::serialize(&self).unwrap();
        println!(
            "Length={} Packet as bytes {:?}",
            packet_header_bytes.len(),
            packet_header_bytes
        );

        packet_header_bytes
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{File, OpenOptions},
        io::Write,
        sync::mpsc,
    };

    use super::*;

    #[test]
    fn test_print_devices() {
        print_devices();
    }

    #[test]
    #[ignore]
    fn test_lookup_default_device() {
        let expected_default_device = lookup_default_device();
        assert_eq!(expected_default_device.name, "en0");
    }

    #[test]
    fn test_expected_create_global_header() {
        let global_header = GlobalHeader {
            magic_number: 3569595041,
            version_major: 2,
            version_minor: 2,
            thiszone: 0,
            sigfigs: 0,
            snaplen: 65535,
            network: 1,
        };

        assert_eq!(
            format!("{global_header}"),
            "(
            magic_number=3569595041, 
            version_major=2, 
            version_minor=2, 
            thiszone=0, 
            sigfigs=0, 
            snaplen=65535, 
            network=1
        )"
        );

        let bytes = bincode::serialize(&global_header).unwrap();
        assert_eq!(24, bytes.len());
        assert!([
            161, 178, 195, 212, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 1, 0, 0, 0
        ]
        .iter()
        .eq(bytes.iter()));

        // let mut f = File::create("test-data.pcap").unwrap();
        // bincode::serialize_into(&mut f, &global_header).unwrap();
    }

    #[test]
    fn test_expected_create_pcap_file() {
        let (tx, rx) = mpsc::channel();

        let global_header = create_global_header();
        println!("Global Header {}", global_header);
        // tx.send(global_header.as_bytes()).unwrap();

        File::create("target/test-data.pcap").unwrap();
        let mut f = OpenOptions::new()
            // .create_new(true)
            .write(true)
            .append(true)
            .open("target/test-data.pcap")
            .unwrap();
        f.write_all(&global_header.as_bytes()).unwrap();

        capture_packages(10, |cnt, packet| tx.send(packet.as_bytes()).unwrap());

        // File::create("target/test-data.pcap").unwrap();
        // let mut f = OpenOptions::new()
        //     // .create_new(true)
        //     .write(true)
        //     .append(true)
        //     .open("target/test-data.pcap")
        //     .unwrap();

        for received in rx {
            println!("received data: length={}, {:?} ", received.len(), received);
            f.write_all(&received).unwrap();
        }
    }
}
