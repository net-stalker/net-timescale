use std::sync::Arc;
use crate::transport::sockets::Handler;
use crate::transport::zmq::connectors::publisher::PubConnectorZmq;

pub struct ConnectorZmqPublisherBuilder<HANDLER: Handler> {
    context: zmq::Context,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
}

impl<HANDLER: Handler> ConnectorZmqPublisherBuilder<HANDLER> {
    pub fn new(context: zmq::Context) -> Self {
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
        let socket = self.context.socket(zmq::PUB).unwrap();
        PubConnectorZmq::new(
            self.endpoint.as_ref().unwrap().to_string(),
            self.handler.as_ref().unwrap().clone(),
            socket,
        )
    }
}

