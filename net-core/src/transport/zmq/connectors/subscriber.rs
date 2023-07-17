use std::cell::RefCell;
use std::os::fd::RawFd;
use std::sync::Arc;
use log::trace;
use zmq::Socket;
use crate::transport::sockets::{Handler, Receiver, Sender, self, Pub};

pub struct SubConnectorZmq<HANDLER: Handler> {
    endpoint: String,
    handler: Arc<HANDLER>,
    socket: zmq::Socket,
}

impl<HANDLER: Handler> Receiver for SubConnectorZmq<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        trace!("receiving data");
        self.socket.recv_bytes(0)
            .expect("connector failed receiving topic");
        self.socket.recv_bytes(0)
            .expect("connector failed receiving data")
    }
}

impl<HANDLER: Handler> Sender for SubConnectorZmq<HANDLER> {
    fn send(&self, _data: &[u8]) {
        panic!("can't send _data via sub socket");
    }
}

impl<HANDLER: Handler> sockets::Socket for SubConnectorZmq<HANDLER> {
    fn as_raw_fd(&self) -> RawFd {
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

impl<HANDLER: Handler> sockets::ZmqSocket for SubConnectorZmq<HANDLER> {
    fn get_socket(&self) -> &Socket {
        &self.socket
    }
}

impl<HANDLER: Handler> SubConnectorZmq<HANDLER> {
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
        Arc::new(self)
    }
}