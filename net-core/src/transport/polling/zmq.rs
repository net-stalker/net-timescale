use std::sync::Arc;
use zmq::PollEvents;
use crate::transport::sockets::ZmqSocket;

pub struct ZmqPoller {
    // TODO: in case we would have multiple zmq connectors there is a point to implement connector trait
    // or think about implementing generic poller using. For example, implement Poll trait which
    // will be implemented by connectors
    sockets: Vec<Arc<dyn ZmqSocket>>,
}

impl ZmqPoller {
    pub fn new() -> Self {
        ZmqPoller { sockets: Vec::new() }
    }
    pub fn add(&mut self, socket: Arc<dyn ZmqSocket>) -> &mut Self {
        self.sockets.push(socket);
        self
    }
    pub fn poll(&mut self, poll_count: i32) {
        // FIXME: remove constructing items into add method. Probably vector of pair will be good
        let mut items = Vec::new();
        let mut counter = 0;
        for socket in &self.sockets {
            let poll_item = socket.get_socket().as_poll_item(PollEvents::POLLIN);
            items.push(poll_item);
        }
        while counter != poll_count {
            zmq::poll(&mut items, -1).expect("polling error");
            for (index, item) in items.iter().enumerate() {
                if item.is_readable() {
                    counter += 1;
                    let socket = &self.sockets[index];
                    socket.handle(socket.get_receiver(), socket.get_sender());
                }
            }
        }
    }
}

mod tests {
    use std::sync::Arc;
    use crate::transport::zmq::connectors::dealer::DealerConnectorZmq;
    use crate::transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
    use crate::transport::polling::zmq::ZmqPoller;
    use crate::transport::sockets::{Handler, Receiver, Sender};

    pub struct ClientCommand;
    impl Handler for ClientCommand {
        fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
            println!("in client handler");
            let msg = receiver.recv();
            assert_eq!(msg, "from server".as_bytes());
        }
    }
    pub struct ServerCommand;
    impl Handler for ServerCommand {
        fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
            println!("In server handle");
            let msg = receiver.recv();
            assert_eq!(msg, "from client".as_bytes());
        }
    }
    fn run_server_zmq(context: zmq::Context) {
        let server = ConnectorZmqDealerBuilder::new(context)
            .with_endpoint("tcp://127.0.0.1:7000".to_string())
            .with_handler(Arc::new(ServerCommand))
            .build()
            .bind()
            .into_inner();
        for _ in 0..5 {
            server.send("from server".as_bytes());
        }
    }
    fn run_client_zmq(context: zmq::Context) {
        let client = ConnectorZmqDealerBuilder::new(context)
            .with_endpoint("tcp://127.0.0.1:7001".to_string())
            .with_handler(Arc::new(ClientCommand))
            .build()
            .connect()
            .into_inner();
        for _ in 0..5 {
            client.send("from client".as_bytes());
        }
    }
    #[test]
    fn zmq_poller_server_test() {
        let zmq_context = zmq::Context::new();
        let clients_count = 3;
        let server = ConnectorZmqDealerBuilder::new(zmq_context.clone())
            .with_endpoint("tcp://127.0.0.1:7001".to_string())
            .with_handler(Arc::new(ServerCommand))
            .build()
            .bind()
            .into_inner();
        let mut clients = Vec::new();
        for _ in 0..clients_count {
            let context_clone = zmq_context.clone();
            clients.push(std::thread::spawn(move || {
                run_client_zmq(context_clone);
            }));
        }
        ZmqPoller::new()
            .add(server.clone())
            .poll(clients_count * 2);

        clients.into_iter().for_each(|client| {
            client.join().unwrap();
        });
    }
    #[test]
    fn zmq_poller_client_test() {
        let zmq_context = zmq::Context::new();
        let context_clone = zmq_context.clone();
        let server = std::thread::spawn(move || {
            run_server_zmq(context_clone);
        });
        let client = ConnectorZmqDealerBuilder::new(zmq_context.clone())
            .with_endpoint("tcp://127.0.0.1:7000".to_string())
            .with_handler(Arc::new(ClientCommand))
            .build()
            .connect()
            .into_inner();

        ZmqPoller::new()
            .add(client)
            .poll(5);

        server.join().unwrap();
    }
}