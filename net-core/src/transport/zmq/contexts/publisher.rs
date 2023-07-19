use crate::transport::sockets::Context;

#[derive(Default, Clone)]
pub struct PublisherContext {
    context: zmq::Context,
}

impl Context for PublisherContext {
    type S = zmq::Socket;
    type C = zmq::Context;

    fn create_socket(&self) -> Self::S {
        self.context.socket(zmq::PUB).unwrap()
    }
    fn get_context(&self) -> Self::C { self.context.clone() }
}

impl PublisherContext {
    pub fn new(context: zmq::Context) -> Self {
        Self { context, }
    }
}