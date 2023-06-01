use std::collections::HashMap;


use std::sync::Arc;


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
        self.sockets.insert(socket.as_raw_fd(), socket);

        self
    }

    pub fn poll(&mut self, events_count: i32) {
        let mut counter = 0;
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
                counter += 1;
                let socket = self.sockets.get(&(ev.key as i32)).unwrap();
                socket.handle(socket.get_receiver(), socket.get_sender());

                poller.modify(socket.as_raw_fd(), Event::readable(ev.key)).unwrap();
                
                if counter == events_count { return; }
            }
        }
    }
}

mod tests {
    use std::{thread, sync::Mutex, time::Duration};
    use crate::transport::{
        {
        connector_nng::{ConnectorNNG, Proto},
        connector_zeromq::{
            ConnectorZmq
        }
    },
    };
    use crate::transport::polling::Poller;
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
            Poller::new()
                .add(server)
                .add(client)
                .poll(2);
        }).join().unwrap();
    }

    fn run_zmq_server() {
        let server = ConnectorZmq::builder()
            .with_endpoint("tcp://127.0.0.1:7000".to_string())
            .with_handler(ServerCommand)
            .build()
            .bind()
            .into_inner();
        thread::sleep(Duration::from_secs(1));
        for _ in 0..5 {
            server.send("from server".as_bytes());
        }
        thread::sleep(Duration::from_secs(2));
    }
    fn run_zmq_client() {
        let client = ConnectorZmq::builder()
            .with_endpoint("tcp://127.0.0.1:7001".to_string())
            .with_handler(ClientCommand)
            .build()
            .connect()
            .into_inner();
        thread::sleep(Duration::from_secs(1));
        for _ in 0..5 {
            client.send("from client".as_bytes());
        }
        thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn zeromq_dealer_client_polling_test() {
        let server = std::thread::spawn(run_zmq_server);
        let client = ConnectorZmq::builder()
            .with_endpoint("tcp://127.0.0.1:7000".to_string())
            .with_handler(ClientCommand)
            .build()
            .connect()
            .into_inner();

        Poller::new()
            .add(client)
            .poll(1);
        
        server.join().unwrap();
    }
    #[test]
    fn zeromq_dealer_server_polling_test() {
        let server = ConnectorZmq::builder()
            .with_endpoint("tcp://127.0.0.1:7001".to_string())
            .with_handler(ServerCommand)
            .build()
            .bind()
            .into_inner();
        let client = std::thread::spawn(run_zmq_client); 
        Poller::new()
            .add(server)
            .poll(1);

        client.join().unwrap();
    }
}