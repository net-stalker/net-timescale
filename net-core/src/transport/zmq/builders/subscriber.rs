use std::sync::Arc;
use crate::transport::sockets::Handler;
use crate::transport::zmq::connectors::subscriber::SubConnectorZmq;

pub struct ConnectorZmqSubscriberBuilder<HANDLER: Handler> {
    context: zmq::Context,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
    topic: Vec<u8>,
}

impl<HANDLER: Handler> ConnectorZmqSubscriberBuilder<HANDLER> {
    pub fn new(context: zmq::Context) -> Self {
        ConnectorZmqSubscriberBuilder {
            context,
            endpoint: None,
            handler: None,
            topic: Vec::default(),
        }
    }
    pub fn with_handler(mut self, handler: Arc<HANDLER>) -> Self {
        self.handler = Some(handler);
        self
    }
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
    pub fn with_topic(mut self, topic: Vec<u8>) -> Self {
        self.topic = topic;
        self
    }
    pub fn build(self) -> SubConnectorZmq<HANDLER> {
        let socket = self.context.socket(zmq::SUB).unwrap();
        socket.set_subscribe(self.topic.as_slice()).unwrap();
        SubConnectorZmq::new(
            self.endpoint.as_ref().unwrap().to_string(),
            self.handler.as_ref().unwrap().clone(),
            socket,
        )
    }
}

