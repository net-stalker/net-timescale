use std::sync::Arc;

use zmq::Socket;

use crate::{CONNECTOR_MONITOR_ENDPOINT, HubContext, PollerSpec};

pub struct Connector {
    connector_socket: Socket,
}

impl Connector {
    pub fn new(hub_context: Arc<HubContext>) -> Self {
        let connector_socket = hub_context.zmq_ctx.socket(zmq::DEALER).unwrap();
        connector_socket.connect(CONNECTOR_MONITOR_ENDPOINT)
            .expect("failed connect to monitor connector endpoint");

        Self { connector_socket }
    }
}

impl PollerSpec for Connector {
    fn poll(&self, handler: impl Fn(Vec<u8>)) {
        let mut items = [self.connector_socket.as_poll_item(zmq::POLLIN)];

        loop {
            let rc = zmq::poll(&mut items, -1).unwrap();
            if rc == -1 {
                break;
            }

            if !items[0].is_readable() {
                return;
            }

            let msg = self.connector_socket
                .recv_bytes(0)
                .expect("monitor manager failed receiving response");
            println!("received from connector {:?}", msg);

            handler(msg);
        }
    }
}