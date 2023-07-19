use std::sync::Arc;
use crate::transport::sockets::{Handler, Context};
use crate::transport::zmq::connectors::dealer::DealerConnectorZmq;

pub struct ConnectorZmqDealerBuilder<'a, HANDLER: Handler> {
    context: &'a dyn Context<S = zmq::Socket, C = zmq::Context>,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
}

impl<'a, HANDLER: Handler> ConnectorZmqDealerBuilder<'a, HANDLER> {
    pub fn new(context: &'a dyn Context<S = zmq::Socket, C = zmq::Context>) -> Self {
        ConnectorZmqDealerBuilder {
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

    pub fn build(mut self) -> DealerConnectorZmq<HANDLER> {
        let socket = self.context.create_socket();
        DealerConnectorZmq::new(
            self.endpoint.as_ref().unwrap().to_string(),
            self.handler.as_ref().unwrap().clone(),
            socket
        )
    }
}

