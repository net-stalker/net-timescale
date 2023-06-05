use std::sync::Arc;
use zmq::PollEvents;
use crate::transport::connector_zeromq::ConnectorZMQ;
use crate::transport::sockets::{Socket, Handler};

pub struct ZmqPoller<HANDLER>
where HANDLER: Handler
{
    // TODO: in case we would have multiple zmq connectors there is a point to implement connector trait
    // or think about implementing generic poller using, for example, Pool trait which is implemented by connectors
    connectors: Vec<Arc<ConnectorZMQ<HANDLER>>>,
}

impl<HANDLER> ZmqPoller<HANDLER>
where HANDLER: Handler
{
    pub fn new() -> Self {
        ZmqPoller { connectors: Vec::new() }
    }
    pub fn add(&mut self, socket: Arc<ConnectorZMQ<HANDLER>>) -> &mut Self {
        self.connectors.push(socket);
        self
    }
    pub fn poll(&mut self, poll_count: i32) {
        let mut items = Vec::new();
        let mut counter = 0;
        for connector in &self.connectors {
            let poll_item = connector.get_socket().as_poll_item(PollEvents::POLLIN);
            items.push(poll_item);
        }
        while counter != poll_count {
            zmq::poll(&mut items, -1).expect("polling error");
            for (index, item) in items.iter().enumerate() {
                if item.is_readable() {
                    counter += 1;
                    let socket = &self.connectors[index];
                    socket.handle(socket.get_receiver(), socket.get_sender());
                }
            }
        }
    }
}

mod tests {

    use crate::transport::{
        connector_zeromq::{
            ConnectorZMQ
        }
    };
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
    fn run_server_zmq() {
        let server = ConnectorZMQ::builder()
            .with_endpoint("tcp://127.0.0.1:7000".to_string())
            .with_handler(ServerCommand)
            .build_dealer()
            .bind()
            .into_inner();
        for _ in 0..5 {
            server.send("from server".as_bytes());
        }
    }
    fn run_client_zmq() {
        let client = ConnectorZMQ::builder()
            .with_endpoint("tcp://127.0.0.1:7001".to_string())
            .with_handler(ClientCommand)
            .build_dealer()
            .connect()
            .into_inner();
        for _ in 0..5 {
            client.send("from client".as_bytes());
        }
    }
    #[test]
    fn zmq_poller_server_test() {
        let clients_count = 3;
        let server = ConnectorZMQ::builder()
            .with_endpoint("tcp://127.0.0.1:7001".to_string())
            .with_handler(ServerCommand)
            .build_dealer()
            .bind()
            .into_inner();
        let mut clients = Vec::new();
        for _ in 0..clients_count {
            clients.push(std::thread::spawn(run_client_zmq));
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
        let server = std::thread::spawn(run_server_zmq);
        let client = ConnectorZMQ::builder()
            .with_endpoint("tcp://127.0.0.1:7000".to_string())
            .with_handler(ClientCommand)
            .build_dealer()
            .connect()
            .into_inner();

        ZmqPoller::new()
            .add(client)
            .poll(5);

        server.join().unwrap();
    }
}