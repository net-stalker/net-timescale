use std::collections::HashMap;


use std::sync::Arc;


use polling::Event;

use crate::transport::sockets::Socket;

pub struct NngPoller {
    sockets: HashMap<i32, Arc<dyn Socket>>,
}

impl NngPoller {
    pub fn new() -> NngPoller {
        NngPoller { sockets: HashMap::new() }
    }

    pub fn add<S: Socket + 'static>(&mut self, socket: Arc<S>) -> &mut Self {
        self.sockets.insert(socket.as_raw_fd(), socket);
        self
    }

    pub fn poll(&mut self) {
        let poller = polling::Poller::new().unwrap();
        let mut events = Vec::new();

        self.sockets.values().for_each(|socket| {
            let usize_fd = socket.fd_as_usize().unwrap();
            let event = Event::readable(usize_fd);
            let fd = socket.as_raw_fd();

            poller.add(fd, event).unwrap();
        });

        loop {
            events.clear();
            poller.wait(&mut events, None).unwrap();

            for ev in &events {
                let socket = self.sockets.get(&(ev.key as i32)).unwrap();
                socket.handle(socket.get_receiver(), socket.get_sender());

                poller.modify(socket.as_raw_fd(), Event::readable(ev.key)).unwrap();
            }
        }
    }
    fn poll_with_limit(&mut self, mut events_count: i32) {
        let poller = polling::Poller::new().unwrap();
        let mut events = Vec::new();

        self.sockets.values().for_each(|socket| {
            let usize_fd = socket.fd_as_usize().unwrap();
            let event = Event::readable(usize_fd);
            let fd = socket.as_raw_fd();

            poller.add(fd, event).unwrap();
        });

        loop {
            events.clear();
            poller.wait(&mut events, None).unwrap();

            for ev in &events {
                events_count -= 1;
                let socket = self.sockets.get(&(ev.key as i32)).unwrap();
                socket.handle(socket.get_receiver(), socket.get_sender());
                
                poller.modify(socket.as_raw_fd(), Event::readable(ev.key)).unwrap();
                if events_count == 0 {
                    return;
                }
            }
        }
    }
}

mod tests {
    use super::*;
    use std::{thread, time::Duration};
    use crate::transport::{
        connector_nng::{ConnectorNNG, Proto},
        connector_zeromq::{
            ConnectorZmqBuilder,
            ConnectorZMQ
        }
    };
    use crate::transport::sockets::{Handler, Receiver, Sender};

    struct ClientCommand;
    impl Handler for ClientCommand {
        fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
            let msg = receiver.recv();
            assert_eq!(msg, "from server".as_bytes());
        }
    }
    struct ServerCommand;
    impl Handler for ServerCommand {
        fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
            let msg = receiver.recv();
            assert_eq!(msg, "from client".as_bytes());
        }
    }

    #[test]
    fn expected_create_poller_using_builder() {
        struct ClientCommand;
        impl Handler for ClientCommand {
            fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
                let msg = receiver.recv();
                assert_eq!(&msg[..], b"msg1");
            }
        }

        let client = ConnectorNNG::builder()
            .with_endpoint("ws://127.0.0.1:5555".to_string())
            .with_proto(Proto::Req)
            .with_handler(ClientCommand)
            .build()
            .connect()
            .into_inner();

        let arc = client.clone();
        thread::spawn(move || {
            arc.send(b"msg1");
        });

        struct ServerCommand;
        impl Handler for ServerCommand {
            fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
                let data = receiver.recv();
                println!("We got a message: {:?}", data);
                sender.send(data.as_slice());
            }
        }

        let server = ConnectorNNG::builder()
            .with_endpoint("ws://127.0.0.1:5555".to_string())
            .with_proto(Proto::Rep)
            .with_handler(ServerCommand)
            .build()
            .bind()
            .into_inner();

        thread::spawn(move || {
            NngPoller::new()
                .add(server)
                .add(client)
                .poll_with_limit(2);
        }).join().unwrap();
    }
}