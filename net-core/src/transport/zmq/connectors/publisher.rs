use std::cell::RefCell;
use std::os::fd::RawFd;
use std::sync::Arc;
use log::trace;
use zmq::Socket;
use crate::transport::sockets::{Handler, Receiver, Sender, self, Pub};

pub struct PubConnectorZmq<HANDLER: Handler> {
    endpoint: String,
    handler: Arc<HANDLER>,
    socket: zmq::Socket,
    topic: RefCell<Vec<u8>>,
}

impl<HANDLER: Handler> Receiver for PubConnectorZmq<HANDLER> {
    // TODO: probably there is a sense to modify receiver trait to make it return Result<Vec<u8>>
    fn recv(&self) -> Vec<u8> {
        panic!("can't receive data in pub socket");
    }
}

impl<HANDLER: Handler> Sender for PubConnectorZmq<HANDLER> {
    fn send(&self, data: &[u8]) {
        println!("sending data {:?}", data);
        trace!("sending data {:?}", data);
        self.socket.send(self.topic.borrow().as_slice(), zmq::SNDMORE).unwrap();
        self.socket.send(data, 0)
            .expect("client failed sending data");
    }
}

impl<HANDLER: Handler> sockets::Socket for PubConnectorZmq<HANDLER> {
    fn as_raw_fd(&self) -> RawFd {
        self.socket.get_fd().unwrap()
    }

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        self.handler.handle(receiver, sender);
    }

    fn get_receiver(&self) -> &dyn Receiver {
        self
    }

    fn get_sender(&self) -> &dyn Sender {
        self
    }
}
impl<HANDLER: Handler> sockets::Pub for PubConnectorZmq<HANDLER> {
    fn set_topic(&self, topic: &[u8]) {
        self.topic.replace(topic.to_owned());
    }
}
impl<HANDLER: Handler> sockets::ZmqSocket for PubConnectorZmq<HANDLER> {
    fn get_socket(&self) -> &Socket {
        &self.socket
    }
}

impl<HANDLER: Handler> PubConnectorZmq<HANDLER> {
    pub fn new(endpoint: String, handler: Arc<HANDLER>, socket: zmq::Socket) -> Self {
        Self {
            endpoint,
            handler,
            socket,
            topic: RefCell::new(Vec::default()),
        }
    }
    pub fn bind(self) -> Self {
        self.socket.bind(&self.endpoint)
            .expect("couldn't bind a connector");
        self
    }
    pub fn connect(self) -> Self {
        self.socket.connect(&self.endpoint)
            .expect("couldn't establish a connection");
        self
    }
    pub fn into_inner(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    use threadpool::ThreadPool;
    use crate::transport::polling::zmq::ZmqPoller;
    use crate::transport::sockets::{Context, Handler, Pub, Receiver, Sender};
    use crate::transport::zmq::builders::publisher::ConnectorZmqPublisherBuilder;
    use crate::transport::zmq::builders::subscriber::ConnectorZmqSubscriberBuilder;
    use crate::transport::zmq::contexts::publisher::PublisherContext;
    use crate::transport::zmq::contexts::subscriber::SubscriberContext;

    pub const SERVER_URL: &str = "inproc://test/pub-sub/server";

    pub struct ClientCommandTopic1;
    impl Handler for ClientCommandTopic1 {
        fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
            println!("in client1 handler");
            let msg = receiver.recv();
            println!("msg {:?}", msg);
            assert_eq!(msg, "to clients topic1".as_bytes());
        }
    }
    pub struct ClientCommandTopic2;
    impl Handler for ClientCommandTopic2 {
        fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
            println!("in client2 handler");
            let msg = receiver.recv();
            println!("msg {:?}", msg);
            assert_eq!(msg, "to clients topic2".as_bytes());
        }
    }
    pub struct ServerCommand;
    impl Handler for ServerCommand {
        fn handle(&self, _receiver: &dyn Receiver, _sender: &dyn Sender) { }
    }
    fn spawn_clients_topic1(context: &SubscriberContext, count: i32) {
        let mut poller = ZmqPoller::new();
        for _ in 0..count {
            println!("spawning topic1");
            let client = ConnectorZmqSubscriberBuilder::new(&context)
                .with_handler(Arc::new(ClientCommandTopic1))
                .with_endpoint(SERVER_URL.to_string())
                .with_topic("topic1".as_bytes().to_owned())
                .build()
                .connect()
                .into_inner();
            poller.add(client);
        }
        poller.poll(count);
    }
    fn spawn_clients_topic2(context: &SubscriberContext, count: i32) {
        let mut poller = ZmqPoller::new();
        for _ in 0..count {
            println!("spawning topic2");
            let client = ConnectorZmqSubscriberBuilder::new(&context)
                .with_handler(Arc::new(ClientCommandTopic2))
                .with_endpoint(SERVER_URL.to_string())
                .with_topic("topic2".as_bytes().to_owned())
                .build()
                .connect()
                .into_inner();
            poller.add(client);
        }
        poller.poll(count);
    }
    #[test]
    fn test_pub() {
        const CLIENTS_TOPIC1_COUNT: i32 = 3;
        const CLIENTS_TOPIC2_COUNT: i32 = 3;
        let thread_pool = ThreadPool::with_name("pub/sub zmq test".to_string(), 2);
        let pub_context = PublisherContext::default();
        let sub_context = SubscriberContext::new(pub_context.get_context());
        // spawn clients here
        println!("test_pub");
        let sub_context_clone = sub_context.clone();
        thread_pool.execute(move || {
            spawn_clients_topic1(&sub_context_clone, CLIENTS_TOPIC1_COUNT);
        });
        thread_pool.execute(move || {
            spawn_clients_topic2(&sub_context, CLIENTS_TOPIC2_COUNT);
        });
        thread::sleep(Duration::from_millis(500));
        let server = ConnectorZmqPublisherBuilder::new(&pub_context)
            .with_handler(Arc::new(ServerCommand))
            .with_endpoint(SERVER_URL.to_string())
            .build()
            .bind()
            .into_inner();
        server.set_topic("topic1".as_bytes());
        server.send("to clients topic1".as_bytes());
        server.set_topic("topic2".as_bytes());
        server.send("to clients topic2".as_bytes());
        thread_pool.join();
    }
}