use std::ops::Deref;
use std::sync::Arc;
use net_core::capture::global_header::GlobalHeader;
use net_core::capture::packet::Packet;
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
        let global_header = GlobalHeader::new();
        println!("{:?}", global_header);

        let mut buf = global_header.to_bytes();
        buf.append(&mut packet.to_bytes());
        self.client.send(buf)
    }
}