use std::sync::Arc;

use super::sockets::{
    self,
    Receiver,
    Sender,
    Handler,
    Pub
};

pub struct ConnectorZmq<HANDLER: Handler> {
    endpoint: String,
    handler: Arc<HANDLER>,
    _context: zmq::Context,
    socket: zmq::Socket
}

impl<HANDLER: Handler> Receiver for ConnectorZmq<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        self.socket.recv_bytes(0)
            .expect("connector failed receiving data")
    }
}

impl<HANDLER: Handler> Pub for ConnectorZmq<HANDLER> {
    fn set_topic(&self, _topic: &[u8]){
        log::error!("can't set a topic for a non pub connector");
    }
}


impl<HANDLER: Handler> Sender for ConnectorZmq<HANDLER> {
    fn send(&self, data: &[u8]) {
        self.socket.send(data, 0)
            .expect("client failed sending data");
    }

    fn get_pub(&self) -> Option<&dyn Pub> {
        None
    }
}

impl<HANDLER: Handler> sockets::Socket for ConnectorZmq<HANDLER> {
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
impl<HANDLER: Handler> sockets::ZmqSocket for ConnectorZmq<HANDLER> {
    fn get_socket(&self) -> &zmq::Socket {
        &self.socket
    }
}

impl<HANDLER: Handler> ConnectorZmq<HANDLER> {
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
    // TODO: remove builder method from ConnectorZmq and create standalone builders for different patterns
    pub fn builder() -> ConnectorZmqDealerBuilder<HANDLER> {
        ConnectorZmqDealerBuilder::new()
    }
}

pub struct ConnectorZmqDealerBuilder<HANDLER: Handler> {
    context: zmq::Context,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
    socket: Option<zmq::Socket> 
}

impl<HANDLER: Handler> ConnectorZmqDealerBuilder<HANDLER> {
    pub fn new() -> Self {
        ConnectorZmqDealerBuilder {
            context: zmq::Context::new(),
            endpoint: None,
            handler: None,
            socket: None
        }
    }
    pub fn with_handler(mut self, handler: HANDLER) -> Self {
        self.handler = Some(Arc::new(handler));
        self
    }
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
     
    pub fn build(mut self) -> ConnectorZmq<HANDLER> {
        self.socket = Some(self.context.socket(zmq::DEALER).unwrap());
        ConnectorZmq { 
            endpoint: self.endpoint.unwrap(),
            handler: self.handler.unwrap(),
            socket: self.socket.unwrap(),
            _context: self.context
        }
    }
}

