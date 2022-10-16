use std::sync::Arc;

use zmq::{Context, Socket};

use crate::hub_context::HubContext;

pub const CONNECTOR_ENDPOINT: &'static str = "inproc://monitor";

pub struct Manager {
    hub_context: Arc<HubContext>,
    pub monitor_poller: MonitorPoller,
}

impl Manager {
    pub fn new(hub_context: Arc<HubContext>) -> Self {
        let monitor_socket = hub_context.zmq_ctx.socket(zmq::DEALER).unwrap();
        monitor_socket.bind(&hub_context.config.dealer.endpoint)
            .expect("failed bind server");

        let connector_socket = hub_context.zmq_ctx.socket(zmq::DEALER).unwrap();
        connector_socket.bind(CONNECTOR_ENDPOINT)
            .expect("failed bind monitor endpoint");

        Self {
            hub_context: hub_context,
            monitor_poller: MonitorPoller { monitor_socket, connector_socket },
        }
    }
}

pub struct MonitorPoller {
    monitor_socket: Socket,
    connector_socket: Socket,
}

pub trait PollerSpec {
    fn poll(&self, handler: impl Fn(Vec<u8>));
}

impl PollerSpec for MonitorPoller {
    fn poll(&self, handler: impl Fn(Vec<u8>)) {
        let mut items = [self.monitor_socket.as_poll_item(zmq::POLLIN)];

        loop {
            let rc = zmq::poll(&mut items, -1).unwrap();
            if rc == -1 {
                break;
            }

            if !items[0].is_readable() {
                return;
            }

            let msg = self.monitor_socket
                .recv_bytes(0)
                .expect("monitor manager failed receiving response");
            println!("received from monitor {:?}", msg);
            self.connector_socket
                .send(msg, 0)
                .unwrap();
            // handler(msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use net_commons::config::{ConfigManager, ConfigSpec, FileLoader, FileLoaderSpec};

    use super::*;

    #[test]
    fn create_poller() {
        // let config = ConfigManager { application_name: "net-hub", file_loader: Box::new(FileLoader) as Box<dyn FileLoaderSpec> }.load();
        // let poller = Poller { config: config };
        // poller.poll();
        //
        // let ctx = zmq::Context::new();
        // let socket = ctx.socket(zmq::DEALER).unwrap();
        // socket.connect("tcp:localhost:5555");
    }
}