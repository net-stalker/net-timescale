use std::cell::RefCell;
use std::os::unix::io::RawFd;
use std::sync::Arc;

use nng::{Socket, Message, Protocol};
use nng::options::{Options, RecvFd};

use crate::transport::sockets;
use crate::transport::sockets::{Handler, Receiver, Sender};

use super::sockets::Pub;

pub struct ConnectorNNGPubSub<HANDLER> {
    endpoint: String,
    handler: Option<Box<HANDLER>>,
    socket: Socket,
    topic: RefCell<Vec<u8>>
}

impl<HANDLER> Receiver for ConnectorNNGPubSub<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        let data = self.socket.recv().unwrap();
        data[self.topic.borrow().len()..].to_vec()
    }
}
impl<H: Handler> Pub for ConnectorNNGPubSub<H> {
    fn set_topic(&self, topic: &[u8]) {
        self.topic.replace(topic.to_owned()); 
    }
}

impl<H: Handler> Sender for ConnectorNNGPubSub<H> {
    fn send(&self, data: &[u8]) {
        let topic = self.topic.borrow();
        let mut msg = Message::with_capacity(data.len() + topic.len());
        msg.push_back(topic.as_slice());
        msg.push_back(data);
        self.socket
            .send(msg)
            .expect("client failed sending data")
    }
}

impl<HANDLER: Handler> sockets::Socket for ConnectorNNGPubSub<HANDLER>
{
    fn as_raw_fd(&self) -> RawFd {
        //FIXME RecvFd will be worked only for Protocols that can receive data
        self.socket.get_opt::<RecvFd>().unwrap()
    }

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        self.handler.as_ref().unwrap().handle(receiver, sender);
    }

    fn get_receiver(&self) -> &dyn Receiver {
        self
    }

    fn get_sender(&self) -> &dyn Sender {
        self
    }
}

impl<HANDLER: Handler> ConnectorNNGPubSub<HANDLER> {
    pub fn bind(self) -> Self {
        self.socket.listen(&self.endpoint).unwrap();
        self
    }

    pub fn connect(self) -> Self {
        self.socket
            .dial_async(&self.endpoint)
            .expect(format!("failed connecting to {}", &self.endpoint).as_str());

        self
    }

    pub fn into_inner(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub fn builder() -> PubSubConnectorNngBuilder<HANDLER> {
        PubSubConnectorNngBuilder::new()
    }
}

pub struct PubSubConnectorNngBuilder<HANDLER: Handler> {
    endpoint: Option<String>,
    handler: Option<Box<HANDLER>>,
    topics:  Vec<u8>,
    proto: Protocol
}

impl<HANDLER: Handler> PubSubConnectorNngBuilder<HANDLER> {
    pub fn new() -> PubSubConnectorNngBuilder<HANDLER> {
        PubSubConnectorNngBuilder {
            endpoint: None,
            handler: None,
            topics: Vec::<u8>::default(),
            proto: Protocol::Pub0
        }
    }

    pub fn with_handler(mut self, handler: HANDLER) -> Self {
        self.handler = Some(Box::new(handler));
        self
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn with_topic(mut self, topics: Vec<u8>) -> Self {
        self.topics = topics;
        self 
    }

    fn build(self) -> ConnectorNNGPubSub<HANDLER> {
        let socket = Socket::new(self.proto).unwrap();
        if self.proto == Protocol::Sub0 {
            socket.set_opt::<nng::options::protocol::pubsub::Subscribe>(self.topics.clone()).unwrap();
        }
        ConnectorNNGPubSub {
            endpoint: self.endpoint.unwrap(),
            handler: self.handler,
            topic: RefCell::new(self.topics),
            socket
        }
    }

    pub fn build_publisher(mut self) -> ConnectorNNGPubSub<HANDLER> {
        self.proto = Protocol::Pub0;
        self.build()
    }

    pub fn build_subscriber(mut self) -> ConnectorNNGPubSub<HANDLER> {
        self.proto = Protocol::Sub0;
        self.build()
    }
}