use std::collections::HashMap;
use std::num::TryFromIntError;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use nng::options::{Options, RecvFd};
use polling::{Event};

pub trait Socket {
    fn fd(&self) -> RawFd;

    fn fd_as_usize(&self) -> Result<usize, TryFromIntError>;

    fn recv(&self) -> Vec<u8>;

    fn handle(&self, data: Vec<u8>);
}

pub struct Poller {
    sockets: HashMap<i32, Box<dyn Socket>>,
}

impl Poller {
    fn new() -> Poller {
        Poller { sockets: HashMap::new() }
    }

    pub fn add<S: Socket + 'static>(&mut self, socket: S) -> &mut Self {
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
                let mut msg = socket.recv();
                socket.handle(msg);

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
    use crate::transport::connector_nng::Sender;
    use crate::transport::polling::{Poller};

    const ADDRESS: &'static str = "ws://127.0.0.1:5555";

    #[test]
    fn expected_create_poller_using_builder() {
        let handle0 = thread::spawn(move || {
            // Set up the client and connect to the specified address
            let client = Socket::new(Protocol::Req0).unwrap();
            client.dial_async(ADDRESS).unwrap();

            // Send the request from the client to the server. In general, it will be
            // better to directly use a `Message` to enable zero-copy, but that doesn't
            // matter here.
            client.send("Ferris1".as_bytes()).unwrap();
            client.send("Ferris2".as_bytes()).unwrap();

            // Wait for the response from the server.
            let msg = client.recv().unwrap();
            assert_eq!(&msg[..], b"Hello, Ferris");
        });

        let server = connector_nng::ConnectorBuilder::new()
            .with_xtype(zmq::DEALER)
            .with_endpoint("inproc://test".to_string())
            .with_handler(|data| {
                // let result = String::from_utf8(data);
                // println!("received data {:?}", result);
                println!("We got a message: {:?}", data);
                // socket.send(msg).unwrap();
            })
            .build()
            .bind();

        let handle = thread::spawn(move || {
            Poller::new()
                .add(server)
                .poll();
        });

        handle0.join().unwrap();
        // handle.join().unwrap();
    }

    #[test]
    fn play_with_nng() {
        use nng::*;
        use nng::options::{Options, Raw, RecvFd};

        let handle = thread::spawn(move || {
            // Set up the client and connect to the specified address
            let client = Socket::new(Protocol::Req0).unwrap();
            client.dial_async(ADDRESS).unwrap();

            // Send the request from the client to the server. In general, it will be
            // better to directly use a `Message` to enable zero-copy, but that doesn't
            // matter here.
            client.send("Ferris1".as_bytes()).unwrap();
            client.send("Ferris2".as_bytes()).unwrap();

            // Wait for the response from the server.
            let msg = client.recv().unwrap();
            assert_eq!(&msg[..], b"Hello, Ferris");
        });

        let handle_2 = thread::spawn(move || {
            // Set up the server and listen for connections on the specified address.
            let socket = Socket::new(Protocol::Rep0).unwrap();
            socket.listen(ADDRESS).unwrap();
            let raw = socket.get_opt::<RecvFd>().unwrap();

            let poller = polling::Poller::new().unwrap();
            let key = 8;
            poller.add(&raw, Event::readable(key)).unwrap();
            let mut events = Vec::new();

            loop {
                events.clear();
                poller.wait(&mut events, None).unwrap();

                for ev in &events {
                    if ev.key == key {
                        // Perform a non-blocking accept operation.
                        // socket.accept()?;
                        // Set interest in the next readability event.

                        // Receive the message from the client.
                        let mut msg = socket.recv().unwrap();
                        println!("We got a message: {:?}", msg);
                        // msg.clear();
                        // Reuse the message to be more efficient.
                        msg.push_front(b"Hello, ");

                        socket.send(msg).unwrap();

                        poller.modify(&raw, Event::readable(key)).unwrap();
                    }
                }
            }
        });

        handle.join().unwrap();
        handle_2.join().unwrap();
    }
}