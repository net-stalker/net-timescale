use std::fmt;
use std::num::TryFromIntError;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Arc;

use pcap::{Capture, Device, Packet, PacketCodec, PacketHeader};
use serde::{Deserialize, Serialize};

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
}
