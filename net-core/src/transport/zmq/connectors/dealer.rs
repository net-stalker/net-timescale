use std::sync::Arc;
use log::{debug, trace};

use crate::transport::sockets::{
    self,
    Receiver,
    Sender,
    Handler,
};

pub struct DealerConnectorZmq<HANDLER: Handler> {
    endpoint: String,
    handler: Arc<HANDLER>,
    socket: zmq::Socket,
}

impl<HANDLER: Handler> Receiver for DealerConnectorZmq<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        trace!("receiving data");
        self.socket.recv_bytes(0)
            .expect("connector failed receiving data")
    }
}

impl<HANDLER: Handler> Sender for DealerConnectorZmq<HANDLER> {
    fn send(&self, data: &[u8]) {
        trace!("sending data {:?}", data);
        self.socket.send(data, 0)
            .expect("client failed sending data");
    }
}

impl<HANDLER: Handler> sockets::Socket for DealerConnectorZmq<HANDLER> {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        self.socket.get_fd().unwrap()
    }

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        self.handler.handle(receiver, sender);
    }

    fn get_receiver(&self) -> &dyn Receiver {
        self
    }

    fn get_sender(&self) -> &dyn Sender {
        self
    }
}
impl<HANDLER: Handler> sockets::ZmqSocket for DealerConnectorZmq<HANDLER> {
    fn get_socket(&self) -> &zmq::Socket {
        &self.socket
    }
}

impl<HANDLER: Handler> DealerConnectorZmq<HANDLER> {
    pub fn new(endpoint: String, handler: Arc<HANDLER>, socket: zmq::Socket) -> Self {
        Self {
            endpoint,
            handler,
            socket,
        }
    }
    pub fn bind(self) -> Self {
        self.socket.bind(&self.endpoint)
            .expect("couldn't bind a connector");
        self
    }
    pub fn connect(self) -> Self {
        self.socket.connect(&self.endpoint)
            .expect("couldn't establish a connection");
        self
    }
    pub fn into_inner(self) -> Arc<Self> {
        Arc::from(self)
    }
}
