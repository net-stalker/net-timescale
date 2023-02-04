use std::ops::Deref;
use std::sync::Arc;
use net_core::capture::packet::Packet;
use net_core::capture::pcapture::create_global_header;
use net_core::capture::polling::Handler;
use net_core::transport::connector_nng::ConnectorNNG;
use net_core::transport::sockets::Sender;
use crate::command::dummy::DummyCommand;

pub struct Codec {
    client: Arc<ConnectorNNG<DummyCommand>>,
}

impl Codec {
    pub fn new(client: Arc<ConnectorNNG<DummyCommand>>) -> Codec {
        Codec { client }
    }
}

impl Handler for Codec {
    fn decode(&self, _cnt: i32, packet: Packet) {
        let global_header = create_global_header();
        // println!("Global Header {}", global_header);
        //Send first packet as Global Header of pcap file
        // client_clone.send(global_header.as_bytes());
        // client.send(global_header.as_bytes());

        let mut buf = global_header.as_bytes();
        buf.append(&mut packet.to_bytes());
        self.client.send(buf)
    }
}