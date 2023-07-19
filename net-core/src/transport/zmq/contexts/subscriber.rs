use crate::transport::sockets::Context;

#[derive(Default, Clone)]
pub struct SubscriberContext {
    context: zmq::Context,
}

impl Context for SubscriberContext {
    type S = zmq::Socket;
    type C = zmq::Context;

    fn create_socket(&self) -> Self::S {
        self.context.socket(zmq::SUB).unwrap()
    }
    fn get_context(&self) -> Self::C { self.context.clone() }
}

impl SubscriberContext {
    pub fn new(context: zmq::Context) -> Self {
        Self { context, }
    }
}