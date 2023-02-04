use std::fmt;
use std::num::TryFromIntError;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Arc;

use pcap::{Capture, Device, Packet, PacketCodec, PacketHeader};
use serde::{Deserialize, Serialize};
use crate::file::Reader;

use crate::capture::pcapture::config::Data;
use crate::transport::sockets::{Receiver, Sender, Socket};

pub mod config {
    use derivative::Derivative;
    use serde::{Deserialize, Serialize};

    #[derive(Derivative)]
    #[derive(Serialize, Deserialize, Debug)]
    #[derivative(Default)]
    pub struct Data {
        #[allow(dead_code)]
        #[derivative(Default(value = "[\"en0\".to_string()].to_vec()"))]
        pub devices: Vec<String>,
        #[allow(dead_code)]
        #[derivative(Default(value = "-1"))]
        pub number_packages: i32,
        #[allow(dead_code)]
        #[derivative(Default(value = "1000"))]
        pub buffer_size: i32,
    }
}

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
    pub magic_number: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub thiszone: u32,
    pub sigfigs: u32,
    pub snaplen: u32,
    pub network: u32,
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

#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io::Write, sync::mpsc, thread};
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use crate::transport::polling::Poller;

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

        // let mut f = File::create("../net-core/captures/arp.pcap").unwrap();
        // bincode::serialize_into(&mut f, &global_header).unwrap();
    }

    #[test]
    fn capturing() {
        struct Codec;
        impl PacketCodec for Codec {
            type Item = ();

            fn decode(&mut self, packet: Packet) -> Self::Item {
                todo!()
            }
        }

        thread::spawn(move || {
            // let dev = pcap::Device::lookup()
            //     .expect("device lookup failed")
            //     .unwrap();
            // /// let cap1 = Capture::from_device(dev);
            // let stream = pcap::Capture::from_device(dev)
            //     .unwrap()
            //     // .promisc(true)
            //     // .snaplen(65535)
            //     .buffer_size(1000)
            //     .open()
            //     .unwrap()
            //     .setnonblock()
            //     .unwrap()
            //     .stream(Codec)
            //     .unwrap();
        }).join().unwrap()
    }
}
