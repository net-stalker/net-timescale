use std::sync::Arc;
use crate::{
    transport::{
        sockets::{
            Handler,
            Context,
        },
        zmq::{
            connectors::publisher::PubConnectorZmq,
            contexts::publisher::PublisherContext,
        }
    }
};

pub struct ConnectorZmqPublisherBuilder<'a, HANDLER: Handler> {
    context: &'a PublisherContext,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
}

impl<'a, HANDLER: Handler> ConnectorZmqPublisherBuilder<'a, HANDLER> {
    pub fn new(context: &'a PublisherContext) -> Self {
        ConnectorZmqPublisherBuilder {
            context,
            endpoint: None,
            handler: None,
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
    pub fn build(self) -> PubConnectorZmq<HANDLER> {
        let socket = self.context.create_socket();
        PubConnectorZmq::new(
            self.endpoint.as_ref().unwrap().to_string(),
            self.handler.as_ref().unwrap().clone(),
            socket,
        )
    }
}

