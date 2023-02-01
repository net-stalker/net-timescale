use std::collections::HashMap;
use std::num::TryFromIntError;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use nng::options::{Options, RecvFd};
use polling::Event;
use crate::transport::sockets::Socket;

pub struct Poller {
    sockets: HashMap<i32, Arc<dyn Socket>>,
}

impl Poller {
    pub fn new() -> Poller {
        Poller { sockets: HashMap::new() }
    }

    pub fn add<S: Socket + 'static>(&mut self, socket: Arc<S>) -> &mut Self {
        self.sockets.insert(socket.fd(), socket);

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
    use std::sync::Arc;
    use std::thread;
    use nng::options::{Options, RecvFd};
    use nng::{Aio, Protocol, Socket};
    use polling::Event;
    use crate::transport::{connector_nng, sockets};
    use crate::transport::connector_nng::Proto;
    use crate::transport::polling::Poller;
    use crate::transport::sockets::{Handler, Receiver, Sender};

    #[test]
    fn expected_create_poller_using_builder() {
        struct ClientCommand;
        impl Handler for ClientCommand {
            fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
                let msg = receiver.recv();
                assert_eq!(&msg[..], b"Hello, Ferris");
            }
        }

        let client = connector_nng::ConnectorNng::builder()
            .with_endpoint("ws://127.0.0.1:5555".to_string())
            .with_proto(Proto::Req)
            .with_handler(ClientCommand)
            .build()
            .connect()
            .into_inner();

        let arc = client.clone();
        let client_handle = thread::spawn(move || {
            arc.send(Vec::from("Ferris1"));
            arc.send(Vec::from("Ferris2"));
        });

        struct ServerCommand;
        impl Handler for ServerCommand {
            fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
                let data = receiver.recv();
                println!("We got a message: {:?}", data);
                sender.send(data);
            }
        }

        let server = connector_nng::ConnectorNng::builder()
            .with_endpoint("ws://127.0.0.1:5555".to_string())
            .with_proto(Proto::Rep)
            .with_handler(ServerCommand)
            .build()
            .bind()
            .into_inner();

        let poller = thread::spawn(move || {
            Poller::new()
                .add(server)
                .add(client)
                .poll();
        });

        poller.join().unwrap();
        client_handle.join().unwrap();
    }
}