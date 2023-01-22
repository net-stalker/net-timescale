use std::sync::Arc;

use rand::{Rng, thread_rng};
use zmq::{Socket, SocketType};

use crate::transport::context::{Context, ContextBuilder};

//TODO Connector Builder should be redesigned as Fluent API with constraints.

pub trait Sender {
    fn send(&self, data: Vec<u8>);
}

pub trait Poller {
    fn poll(self);
}

pub struct Connector {
    endpoint: String,
    handler: fn(Vec<u8>),
    socket: Socket,
}

impl Connector {
    pub fn bind(self) -> Connector {
        self.socket
            .bind(&self.endpoint)
            .expect(format!("failed binding on {}", &self.endpoint).as_str());

        self
    }

    pub fn connect(self) -> Connector {
        self.socket
            .connect(&self.endpoint)
            .expect(format!("failed connecting to {}", &self.endpoint).as_str());

        self
    }
}

impl Sender for Connector {
    fn send(&self, data: Vec<u8>) {
        self.socket
            .send(data, 0)
            .expect("client failed sending data");
    }
}

impl Poller for Connector {
    fn poll(self) {
        let mut items = [self.socket.as_poll_item(zmq::POLLIN)];

        loop {
            let rc = zmq::poll(&mut items, -1).unwrap();
            if rc == -1 {
                break;
            }

            if !items[0].is_readable() {
                break;
            }

            let data = self.socket
                .recv_bytes(0)
                .unwrap();
            (self.handler)(data);
        }
    }
}

pub struct ConnectorBuilder {
    context: Arc<Context>,
    identity: String,
    xtype: SocketType,
    endpoint: String,
    handler: fn(Vec<u8>),
}

impl ConnectorBuilder {
    pub fn new() -> ConnectorBuilder {
        let mut rng = thread_rng();
        let context = ContextBuilder::new().build();

        ConnectorBuilder {
            context,
            xtype: zmq::DEALER,
            identity: format!("{:04X}-{:04X}", rng.gen::<u16>(), rng.gen::<u16>()),
            endpoint: "inproc://dummy".to_string(),
            handler: |_data| {},
        }
    }

    fn create_socket(self) -> Socket {
        let socket = self.context.xctx().socket(self.xtype).unwrap();
        socket
            .set_identity(self.identity.as_bytes())
            .expect("failed setting client id");

        socket
    }

    pub fn build(self) -> Connector {
        Connector {
            // Potentially clone method is inefficient but it is called only once when Connector is created.
            endpoint: self.endpoint.clone(),
            handler: self.handler,
            socket: self.create_socket(),
        }
    }

    pub fn with_handler(mut self, handler: fn(Vec<u8>)) -> ConnectorBuilder {
        self.handler = handler;
        self
    }

    pub fn with_endpoint(mut self, endpoint: String) -> ConnectorBuilder {
        self.endpoint = endpoint;
        self
    }

    pub fn with_xtype(mut self, xtype: SocketType) -> ConnectorBuilder {
        self.xtype = xtype;
        self
    }

    pub fn with_context(mut self, context: Arc<Context>) -> ConnectorBuilder {
        self.context = context;
        self
    }
}

mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test() {
        let context = ContextBuilder::new().build(); //TODO Use From trait instead of new
        let connector_context = context.clone();

        let server_handle = thread::spawn(move || {
            ConnectorBuilder::new()
                .with_context(context)
                .with_xtype(zmq::DEALER)
                .with_endpoint("inproc://test".to_string())
                .with_handler(|data| {
                    let result = String::from_utf8(data);
                    println!("received data {:?}", result);
                })
                .build()
                .bind()
                .poll();
        });

        let _client = ConnectorBuilder::new()
            .with_context(connector_context)
            .with_xtype(zmq::DEALER)
            .with_endpoint("inproc://test".to_string())
            .build()
            .connect()
            .send(b"test".to_vec());

        // assert_eq!(json_result, std::str::from_utf8(&json_buffer).unwrap());

        // f: impl Fn(i32, PcapPacket)

        // server_handle.join().unwrap();
    }
}
