use std::collections::HashMap;
use std::os::unix::io::RawFd;
use nng::options::{Options, RecvFd};
use nng::Socket;
use polling::{Event, Poller};
use crate::transport::sockets;
use crate::transport::sockets::{get_fd, as_unsize};

pub trait Handler {
    fn handle(&self, data: Vec<u8>);
}

pub struct NetPoller {
    sockets: HashMap<i32, Socket>,
}

impl NetPoller {
    pub fn poll(self) {
        let poller = Poller::new().unwrap();
        let mut events = Vec::new();

        self.sockets.values().for_each(|socket| {
            let fd = get_fd(socket);
            let key = as_unsize(fd);

            poller.add(fd, Event::readable(key)).unwrap();
        });

        loop {
            events.clear();
            poller.wait(&mut events, None).unwrap();

            for ev in &events {
                let socket = self.sockets.get(&(ev.key as i32)).unwrap();
                let mut msg = socket.recv().unwrap();

                println!("We got a message: {:?}", msg.as_slice());
                // Reuse the message to be more efficient.
                msg.push_front(b"Hello, ");

                socket.send(msg).unwrap();

                let fd = get_fd(socket);
                poller.modify(fd, Event::readable(ev.key)).unwrap();
            }
        }
    }

    pub fn builder() -> PollerBuilder {
        PollerBuilder::new()
    }
}

pub struct PollerBuilder {
    sockets: HashMap<i32, Socket>,
}

impl PollerBuilder {
    fn new() -> PollerBuilder {
        PollerBuilder { sockets: HashMap::new() }
    }

    pub fn add(&mut self, socket: Socket) -> &mut Self {
        let fd = get_fd(&socket);
        self.sockets.insert(fd, socket);

        self
    }

    pub fn build(&self) -> NetPoller {
        NetPoller { sockets: self.sockets.clone() }
    }
}

mod tests {
    use std::thread;
    use nng::options::{Options, RecvFd};
    use nng::{Protocol, Socket};
    use polling::Event;
    use crate::transport::polling::{NetPoller, PollerBuilder};

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

        let socket = Socket::new(Protocol::Rep0).unwrap();
        socket.listen(ADDRESS).unwrap();

        let handle = thread::spawn(move || {
            NetPoller::builder()
                .add(socket)
                .build()
                .poll();
        });

        handle0.join().unwrap();
        handle.join().unwrap();
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