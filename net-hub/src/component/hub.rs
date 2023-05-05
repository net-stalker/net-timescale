use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use log::{debug, info};
use simple_websockets::Event;
use threadpool::ThreadPool;
use net_core::{layer::NetComponent, transport::sockets::Sender};

use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;

use crate::command::{server::ServerCommand, dummy::DummyCommand};
use crate::command::pull::PullCommand;
use crate::command::translator::TranslatorCommand;

pub struct Hub {
    pub pool: ThreadPool,
}

impl Hub {
    pub fn new(pool: ThreadPool) -> Self {
        Hub { pool }
    }
}
const PULL: &'static str = "inproc://nng/pull";
impl NetComponent for Hub {
    fn run(self) {
        info!("Run component");

        //Global for the project
        // let config = Arc::new(ConfigManager { application_name: "net-hub", file_loader: Box::new(ConfigFile) as Box<dyn FileReader> }.load());

        // //Global for the project
        // let config = hub_context.clone().config.clone();
        // if !config.dealer.enable {
        //     debug!("Dealer is disabled!");
        //     return;
        // }

        let clients = Arc::new(RwLock::new(HashMap::new()));
        let clients_inner = clients.clone();

        //TODO use instead ws from ConnectoNNG
        self.pool.execute(move || {
            let event_hub = simple_websockets::launch(9091)
                .expect("failed to listen on port 9091");

            loop {
                match event_hub.poll_event() {
                    Event::Connect(client_id, responder) => {
                        info!("A client connected with id #{}", client_id);
                        clients_inner
                            .write()
                            .unwrap()
                            .insert(client_id, responder.clone());

                        //TODO for every websocket conn should be created new zmq socket referenced to the websocket client connection.
                    }
                    Event::Disconnect(client_id) => {
                        info!("Client #{} disconnected.", client_id);
                        clients_inner.write().unwrap().remove(&client_id);
                    }
                    Event::Message(client_id, message) => {
                        debug!(
                            "Received a message from client #{}: {:?}",
                            client_id, message
                        );
                        // let responder = clients.get(&client_id).unwrap();
                        // let responder = clients.write().unwrap().get(&client_id).unwrap();
                        // responder.send(message);
                    }
                }
            }
        });
        self.pool.execute(move || {
            let pull_sub = ConnectorNNG::pub_sub_builder()
                .with_endpoint(PULL.to_string())
                .with_handler(PullCommand { clients })
                .build_subscriber()
                .bind()
                .into_inner();
            let pull_pub = ConnectorNNG::pub_sub_builder()
                .with_endpoint(PULL.to_string())
                .with_handler(DummyCommand)
                .build_publisher()
                .connect()
                .into_inner();

            let translator = ConnectorNNG::pub_sub_builder()
                .with_endpoint("tcp://0.0.0.0:5557".to_string())
                .with_handler(TranslatorCommand)
                .build_publisher()
                .connect()
                .into_inner();

            let server_command = ServerCommand::<dyn Sender> { 
                translator,
                clients: pull_pub
            };
            let server = ConnectorNNG::pub_sub_builder()
                .with_endpoint("tcp://0.0.0.0:5555".to_string())
                .with_handler(server_command)
                .build_subscriber()
                .bind()
                .into_inner();

            Poller::new()
                .add(server)
                .add(pull_sub)
                .poll();
        });
    }
}