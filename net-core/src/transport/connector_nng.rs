use std::num::TryFromIntError;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use nng::{Aio, Protocol, Socket};
use nng::options::{Options, RecvFd};
use zmq::SocketType;
use crate::transport;

use crate::transport::context::{Context, ContextBuilder};
use crate::transport::sockets;
use crate::transport::sockets::{Handler, Receiver, Sender};

//TODO Connector Builder should be redesigned as Fluent API with constraints.

pub struct ConnectorNNG<HANDLER> {
    endpoint: String,
    handler: Option<Box<HANDLER>>,
    socket: Socket,
}

impl<HANDLER> Receiver for ConnectorNNG<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        self.socket.recv()
            .unwrap()
            .as_slice()
            .to_vec() //note: every time data is coped from stack to the heap!
    }
}

impl<H: Handler> Sender for ConnectorNNG<H> {
    fn send(&self, data: Vec<u8>) {
        self.socket
            .send(&data)
            .expect("client failed sending data");
    }
}

impl<HANDLER: Handler> sockets::Socket for ConnectorNNG<HANDLER>
{
    fn fd(&self) -> RawFd {
        //FIXME RecvFd will be worked only for Protocols that can receive data
        self.socket.get_opt::<RecvFd>().unwrap()
    }

    fn fd_as_usize(&self) -> Result<usize, TryFromIntError> {
        usize::try_from(self.fd())
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

impl<HANDLER: Handler> ConnectorNNG<HANDLER> {
    pub fn bind(self) -> ConnectorNNG<HANDLER> {
        self.socket.listen(&self.endpoint).unwrap();
        self
    }

    pub fn connect(self) -> ConnectorNNG<HANDLER> {
        self.socket
            .dial_async(&self.endpoint)
            .expect(format!("failed connecting to {}", &self.endpoint).as_str());

        self
    }

    pub fn into_inner(self) -> Arc<Self> {
        Arc::from(self)
    }

    pub fn builder() -> ConnectorNngBuilder<HANDLER> {
        ConnectorNngBuilder::new()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Proto
{
    Bus,

    // Pair,

    // Pair,

    Pub,

    Pull,

    Push,

    Rep,

    Req,

    Respondent,

    Sub,

    Surveyor,
}

impl Proto {
    pub fn into(proto: Proto) -> Protocol {
        match proto {
            Proto::Bus => { Protocol::Bus0 }
            Proto::Pub => { Protocol::Pub0 }
            Proto::Pull => { Protocol::Pull0 }
            Proto::Push => { Protocol::Push0 }
            Proto::Rep => { Protocol::Rep0 }
            Proto::Req => { Protocol::Req0 }
            Proto::Respondent => { Protocol::Respondent0 }
            Proto::Sub => { Protocol::Sub0 }
            Proto::Surveyor => { Protocol::Surveyor0 }
        }
    }
}

pub struct ConnectorNngBuilder<HANDLER: Handler> {
    endpoint: Option<String>,
    proto: Option<Proto>,
    handler: Option<Box<HANDLER>>,

}

impl<HANDLER: Handler> ConnectorNngBuilder<HANDLER> {
    pub fn new() -> ConnectorNngBuilder<HANDLER> {
        ConnectorNngBuilder {
            endpoint: None,
            proto: None,
            handler: None,
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

    pub fn with_proto(mut self, proto: Proto) -> Self {
        self.proto = Some(proto);
        self
    }

    pub fn build(self) -> ConnectorNNG<HANDLER> {
        let proto = Proto::into(self.proto.unwrap());

        ConnectorNNG {
            endpoint: self.endpoint.unwrap(),
            handler: self.handler,
            socket: Socket::new(proto).unwrap(),
        }
    }
}

mod tests {
    use std::net::TcpListener;
    use std::io::{Read, Write};
    use std::thread;
    use zmq::{DEALER, ROUTER};
    use polling::Event;
    use crate::transport::context::ContextBuilder;

    #[test]
    fn test() {
        let context = ContextBuilder::new().build(); //TODO Use From trait instead of new
        let connector_context = context.clone();

        // let dealer_server = ConnectorBuilder::new()
        //     .with_context(context)
        //     .with_xtype(zmq::DEALER)
        //     .with_endpoint("inproc://test".to_string())
        //     .with_handler(|data| {
        //         let result = String::from_utf8(data);
        //         println!("received data {:?}", result);
        //     })
        //     .build()
        //     .bind();

        let server_handle = thread::spawn(move || {
            // let poller = polling::Poller::new().unwrap();
            // poller.add(&socket, Event::readable(key));
            // poller.poll();
        });

        // let _client = ConnectorBuilder::new()
        //     .with_context(connector_context)
        //     .with_xtype(zmq::DEALER)
        //     .with_endpoint("inproc://test".to_string())
        //     .build()
        //     .connect()
        //     .send(b"test".to_vec());

        // assert_eq!(json_result, std::str::from_utf8(&json_buffer).unwrap());

        // f: impl Fn(i32, PcapPacket)

        // server_handle.join().unwrap();
    }
}
