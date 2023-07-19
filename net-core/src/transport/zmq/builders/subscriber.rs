use std::sync::Arc;
use crate::{
    transport::{
        sockets::{
            Handler,
            Context,
        },
        zmq::{
            connectors::subscriber::SubConnectorZmq,
            contexts::subscriber::SubscriberContext,
        }
    }
};
pub struct ConnectorZmqSubscriberBuilder<'a, HANDLER: Handler> {
    context: &'a SubscriberContext,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
    topic: Vec<u8>,
}

impl<'a, HANDLER: Handler> ConnectorZmqSubscriberBuilder<'a, HANDLER> {
    pub fn new(context: &'a SubscriberContext) -> Self {
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
        let socket = self.context.create_socket();
        socket.set_subscribe(self.topic.as_slice()).unwrap();
        SubConnectorZmq::new(
            self.endpoint.as_ref().unwrap().to_string(),
            self.handler.as_ref().unwrap().clone(),
            socket,
        )
    }
}

