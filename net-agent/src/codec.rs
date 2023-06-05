use std::sync::Arc;

use log::debug;

use net_core::capture::global_header::GlobalHeader;
use net_core::capture::packet::Packet;
use net_core::capture::polling::Handler;
use net_core::transport::connector_nng::ConnectorNNG;
use net_core::transport::dummy_command::DummyCommand;
use net_core::transport::sockets::Sender;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::envelope::envelope::Envelope;
pub struct Codec {
    // TODO: think about adding the topic to which agent sends data to configuration
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
        debug!("{:?}", global_header);
        debug!("{:?}", packet);

        //TODO very slow, should be redesigned in the task CU-861maxexc
        let mut buf = global_header.to_bytes();
        buf.append(&mut packet.to_bytes()); 

        self.client.send(
            Envelope::new(
            "".to_owned(),
            buf
            ).encode()
        .as_slice());
    }
}