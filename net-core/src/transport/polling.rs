use std::collections::HashMap;
use std::num::TryFromIntError;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use nng::options::{Options, RecvFd};
use polling::Event;
use crate::transport::sockets::Socket;

pub struct Poller<SOCKET> {
    sockets: HashMap<i32, Box<SOCKET>>,
}

impl<S: Socket> Poller<S> {
    fn new() -> Poller<S> {
        Poller { sockets: HashMap::new() }
    }

    pub fn add(&mut self, socket: S) -> &mut Self {
        self.sockets.insert(socket.fd(), Box::new(socket));

        self
    }

    pub fn poll(&mut self) {
        let poller = polling::Poller::new().unwrap();
        let mut events = Vec::new();

        self.sockets.values().for_each(|socket| {
            let usize_fd = socket.fd_as_usize().unwrap();
            let event = Event::readable(usize_fd);

            poller.add(socket.fd(), event).unwrap();
        });

        loop {
            events.clear();
            poller.wait(&mut events, None).unwrap();

            for ev in &events {
                let socket = self.sockets.get(&(ev.key as i32)).unwrap();
                socket.handle(socket.get_receiver(), socket.get_sender());

                poller.modify(socket.fd(), Event::readable(ev.key)).unwrap();
            }
        }
    }
}

mod tests {
    use std::thread;
    use nng::options::{Options, RecvFd};
    use nng::{Protocol, Socket};
    use polling::Event;
    use crate::transport::connector_nng;
    use crate::transport::polling::Poller;
    use crate::transport::sockets::{Handler, Receiver, Sender};

    #[test]
    fn expected_create_poller_using_builder() {
        let handle0 = thread::spawn(move || {
            // Set up the client and connect to the specified address
            let client = Socket::new(Protocol::Req0).unwrap();
            client.dial_async("ws://127.0.0.1:5555".to_string().as_str()).unwrap();

            // Send the request from the client to the server. In general, it will be
            // better to directly use a `Message` to enable zero-copy, but that doesn't
            // matter here.
            client.send("Ferris1".as_bytes()).unwrap();
            client.send("Ferris2".as_bytes()).unwrap();

            // Wait for the response from the server.
            let msg = client.recv().unwrap();
            assert_eq!(&msg[..], b"Hello, Ferris");
        });

        struct Command;
        impl Handler for Command {
            fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
                let data = receiver.recv();
                println!("We got a message: {:?}", data);
                sender.send(data);
            }
        }

        let server = connector_nng::ConnectorNng::new()
            // .with_xtype(zmq::DEALER)
            .with_endpoint("ws://127.0.0.1:5555".to_string())
            .with_handler(Command)
            // .build()
            .bind();

        let handle = thread::spawn(move || {
            Poller::new()
                .add(server)
                .poll();
        });

        handle0.join().unwrap();
        handle.join().unwrap();
    }
}