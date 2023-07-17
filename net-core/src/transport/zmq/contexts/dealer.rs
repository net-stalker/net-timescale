use crate::transport::sockets::Context;

#[derive(Default, Clone)]
pub struct DealerContext {
    context: zmq::Context,
}

impl Context for DealerContext {
    type S = zmq::Socket;

    fn create_socket(&self) -> Self::S {
        self.context.socket(zmq::DEALER).unwrap()
    }
}