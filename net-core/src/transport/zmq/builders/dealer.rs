use std::sync::Arc;
use crate::transport::sockets::Handler;
use crate::transport::zmq::connectors::dealer::DealerConnectorZmq;

pub struct ConnectorZmqDealerBuilder<HANDLER: Handler> {
    context: zmq::Context,
    endpoint: Option<String>,
    handler: Option<Arc<HANDLER>>,
}

impl<HANDLER: Handler> ConnectorZmqDealerBuilder<HANDLER> {
    pub fn new(context: zmq::Context) -> Self {
        ConnectorZmqDealerBuilder {
            context,
            endpoint: None,
            handler: None,
        }
    }
    pub fn into_inner(mut self) -> Arc<Self> {
        Arc::new(self)
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
        let socket = self.context.socket(zmq::DEALER).unwrap();
        DealerConnectorZmq::new(
            self.endpoint.as_ref().unwrap().to_string(),
            self.handler.as_ref().unwrap().clone(),
            socket
        )
    }
}

